//! A bounded, single-request fake HTTP provider for local conformance work.

use std::{
    error::Error,
    fmt,
    io::{self, Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::conformance::{ConformanceFixture, HeaderField, ProviderStep};

const ACCEPT_TIMEOUT: Duration = Duration::from_secs(3);
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_REQUEST_BYTES: usize = 128 * 1024;
const MAX_HEADER_BYTES: usize = 32 * 1024;

/// A local fake provider that accepts exactly one bounded request.
pub struct FakeHttpProvider {
    address: SocketAddr,
    worker: JoinHandle<Result<ProviderReport, ProviderError>>,
}

impl FakeHttpProvider {
    /// Spawns a loopback-only provider on an ephemeral port.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the loopback listener cannot be created or configured.
    pub fn spawn(fixture: ConformanceFixture) -> io::Result<Self> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        listener.set_nonblocking(true)?;
        let address = listener.local_addr()?;
        let worker = thread::spawn(move || serve_once(&listener, &fixture));
        Ok(Self { address, worker })
    }

    /// Returns the loopback socket address selected by the operating system.
    #[must_use]
    pub const fn address(&self) -> SocketAddr {
        self.address
    }

    /// Waits for the single-request provider to stop.
    ///
    /// # Errors
    ///
    /// Returns a stable provider error if the request or script fails, including worker panic.
    pub fn wait(self) -> Result<ProviderReport, ProviderError> {
        self.worker.join().map_err(|_| ProviderError::WorkerPanic)?
    }
}

/// Observable, content-free result of one provider run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderReport {
    /// Total bounded request bytes consumed from the socket.
    pub received_request_bytes: usize,
    /// Whether response headers became visible to the client.
    pub response_committed: bool,
    /// Number of raw wire fragments written successfully.
    pub fragments_written: usize,
    /// How the provider stopped.
    pub terminal: ProviderTerminal,
}

/// Terminal state of the one-request fake provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderTerminal {
    /// The fixture completed cleanly.
    Completed,
    /// The fixture deliberately closed the connection.
    ScriptedDisconnect,
    /// The client disconnected while the provider was writing.
    ClientDisconnected,
}

fn serve_once(
    listener: &TcpListener,
    fixture: &ConformanceFixture,
) -> Result<ProviderReport, ProviderError> {
    let deadline = Instant::now() + ACCEPT_TIMEOUT;
    let (mut stream, _) = loop {
        match listener.accept() {
            Ok(connection) => break connection,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
                    return Err(ProviderError::AcceptTimeout);
                }
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) => return Err(ProviderError::Io(error.kind())),
        }
    };
    stream
        .set_nonblocking(false)
        .map_err(|error| ProviderError::Io(error.kind()))?;
    stream
        .set_read_timeout(Some(IO_TIMEOUT))
        .map_err(|error| ProviderError::Io(error.kind()))?;
    stream
        .set_write_timeout(Some(IO_TIMEOUT))
        .map_err(|error| ProviderError::Io(error.kind()))?;

    let received_request_bytes = read_and_validate_request(&mut stream, fixture)?;
    replay_script(&mut stream, fixture, received_request_bytes)
}

fn replay_script(
    stream: &mut TcpStream,
    fixture: &ConformanceFixture,
    received_request_bytes: usize,
) -> Result<ProviderReport, ProviderError> {
    let mut report = ProviderReport {
        received_request_bytes,
        response_committed: false,
        fragments_written: 0,
        terminal: ProviderTerminal::Completed,
    };

    for step in fixture.steps() {
        match step {
            ProviderStep::ResponseHeaders => {
                let response =
                    response_head(fixture.response().status(), fixture.response().headers());
                if !write_visible(stream, response.as_bytes())? {
                    report.terminal = ProviderTerminal::ClientDisconnected;
                    return Ok(report);
                }
                report.response_committed = true;
            }
            ProviderStep::Fragment { data, .. } => {
                if !write_visible(stream, data.as_bytes())? {
                    report.terminal = ProviderTerminal::ClientDisconnected;
                    return Ok(report);
                }
                report.fragments_written = report.fragments_written.saturating_add(1);
            }
            ProviderStep::Delay { milliseconds } => {
                thread::sleep(Duration::from_millis(*milliseconds));
            }
            ProviderStep::Disconnect => {
                stream
                    .shutdown(Shutdown::Both)
                    .map_err(|error| ProviderError::Io(error.kind()))?;
                report.terminal = ProviderTerminal::ScriptedDisconnect;
                return Ok(report);
            }
            ProviderStep::Complete => {
                stream
                    .flush()
                    .map_err(|error| ProviderError::Io(error.kind()))?;
                stream
                    .shutdown(Shutdown::Write)
                    .map_err(|error| ProviderError::Io(error.kind()))?;
                report.terminal = ProviderTerminal::Completed;
                return Ok(report);
            }
        }
    }

    Err(ProviderError::MissingTerminal)
}

fn write_visible(stream: &mut TcpStream, bytes: &[u8]) -> Result<bool, ProviderError> {
    match stream.write_all(bytes).and_then(|()| stream.flush()) {
        Ok(()) => Ok(true),
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::BrokenPipe
                    | io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::ConnectionReset
            ) =>
        {
            Ok(false)
        }
        Err(error) => Err(ProviderError::Io(error.kind())),
    }
}

fn response_head(status: u16, headers: &[HeaderField]) -> String {
    let mut response = format!("HTTP/1.1 {status} Fixture\r\n");
    for header in headers {
        response.push_str(header.name());
        response.push_str(": ");
        response.push_str(header.value());
        response.push_str("\r\n");
    }
    response.push_str("connection: close\r\n\r\n");
    response
}

fn read_and_validate_request(
    stream: &mut TcpStream,
    fixture: &ConformanceFixture,
) -> Result<usize, ProviderError> {
    let mut request = Vec::with_capacity(4 * 1024);
    let mut scratch = [0_u8; 4 * 1024];
    let mut expected_total = None;

    loop {
        let count = stream
            .read(&mut scratch)
            .map_err(|error| ProviderError::Io(error.kind()))?;
        if count == 0 {
            return Err(ProviderError::UnexpectedEndOfRequest);
        }
        if request.len().saturating_add(count) > MAX_REQUEST_BYTES {
            return Err(ProviderError::RequestTooLarge);
        }
        request.extend_from_slice(&scratch[..count]);

        if expected_total.is_none() {
            if let Some(header_end) = find_header_end(&request) {
                if header_end > MAX_HEADER_BYTES {
                    return Err(ProviderError::HeadersTooLarge);
                }
                let content_length = parse_content_length(&request[..header_end])?;
                let total = header_end
                    .checked_add(4)
                    .and_then(|value| value.checked_add(content_length))
                    .ok_or(ProviderError::RequestTooLarge)?;
                if total > MAX_REQUEST_BYTES {
                    return Err(ProviderError::RequestTooLarge);
                }
                expected_total = Some(total);
            } else if request.len() > MAX_HEADER_BYTES {
                return Err(ProviderError::HeadersTooLarge);
            }
        }

        if expected_total.is_some_and(|total| request.len() >= total) {
            break;
        }
    }

    let total = expected_total.ok_or(ProviderError::MalformedRequest("headers"))?;
    request.truncate(total);
    validate_request_bytes(&request, fixture)?;
    Ok(request.len())
}

fn validate_request_bytes(
    request: &[u8],
    fixture: &ConformanceFixture,
) -> Result<(), ProviderError> {
    let header_end = find_header_end(request).ok_or(ProviderError::MalformedRequest("headers"))?;
    let head = std::str::from_utf8(&request[..header_end])
        .map_err(|_| ProviderError::MalformedRequest("header_encoding"))?;
    let mut lines = head.split("\r\n");
    let request_line = lines
        .next()
        .ok_or(ProviderError::MalformedRequest("request_line"))?;
    let mut parts = request_line.split(' ');
    let method = parts
        .next()
        .ok_or(ProviderError::MalformedRequest("method"))?;
    let path = parts
        .next()
        .ok_or(ProviderError::MalformedRequest("path"))?;
    let version = parts
        .next()
        .ok_or(ProviderError::MalformedRequest("version"))?;
    if parts.next().is_some() || !matches!(version, "HTTP/1.0" | "HTTP/1.1") {
        return Err(ProviderError::MalformedRequest("request_line"));
    }
    if method != fixture.request().method() || path != fixture.request().path() {
        return Err(ProviderError::RequestMismatch("target"));
    }

    let received_headers = parse_headers(lines)?;
    for expected in fixture.request().headers() {
        let matches = received_headers.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case(expected.name()) && *value == expected.value()
        });
        if !matches {
            return Err(ProviderError::RequestMismatch("header"));
        }
    }

    let body = &request[header_end + 4..];
    if body != fixture.request().body().as_bytes() {
        return Err(ProviderError::RequestMismatch("body"));
    }
    Ok(())
}

fn parse_headers<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<Vec<(&'a str, &'a str)>, ProviderError> {
    let mut headers = Vec::new();
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or(ProviderError::MalformedRequest("header"))?;
        if name.is_empty() || value.bytes().any(|byte| byte == 0) {
            return Err(ProviderError::MalformedRequest("header"));
        }
        headers.push((name, value.trim_ascii()));
        if headers.len() > 64 {
            return Err(ProviderError::HeadersTooLarge);
        }
    }
    Ok(headers)
}

fn parse_content_length(head: &[u8]) -> Result<usize, ProviderError> {
    let text = std::str::from_utf8(head)
        .map_err(|_| ProviderError::MalformedRequest("header_encoding"))?;
    let mut length = None;
    for line in text.split("\r\n").skip(1) {
        let Some((name, value)) = line.split_once(':') else {
            return Err(ProviderError::MalformedRequest("header"));
        };
        if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err(ProviderError::UnsupportedTransferEncoding);
        }
        if name.eq_ignore_ascii_case("content-length") {
            if length.is_some() {
                return Err(ProviderError::DuplicateContentLength);
            }
            length = Some(
                value
                    .trim_ascii()
                    .parse()
                    .map_err(|_| ProviderError::MalformedRequest("content_length"))?,
            );
        }
    }
    Ok(length.unwrap_or(0))
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

/// A stable fake-provider failure that excludes request and fixture content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderError {
    /// A socket operation failed.
    Io(io::ErrorKind),
    /// No client connected before the fixed deadline.
    AcceptTimeout,
    /// The request exceeded its total byte bound.
    RequestTooLarge,
    /// The request header block exceeded its byte or count bound.
    HeadersTooLarge,
    /// The peer closed before the complete bounded request arrived.
    UnexpectedEndOfRequest,
    /// The request was syntactically invalid.
    MalformedRequest(&'static str),
    /// The request did not match the synthetic fixture.
    RequestMismatch(&'static str),
    /// Chunked or other transfer coding is outside this first fixture scope.
    UnsupportedTransferEncoding,
    /// Multiple content-length fields are refused.
    DuplicateContentLength,
    /// A validated provider script unexpectedly lacked a terminal step.
    MissingTerminal,
    /// The provider worker panicked.
    WorkerPanic,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(kind) => write!(formatter, "fake provider I/O failed: {kind:?}"),
            Self::AcceptTimeout => formatter.write_str("fake provider accept timed out"),
            Self::RequestTooLarge => formatter.write_str("request exceeds the byte limit"),
            Self::HeadersTooLarge => formatter.write_str("request headers exceed a bound"),
            Self::UnexpectedEndOfRequest => formatter.write_str("request ended before completion"),
            Self::MalformedRequest(field) => write!(formatter, "malformed request: {field}"),
            Self::RequestMismatch(field) => {
                write!(formatter, "request does not match fixture: {field}")
            }
            Self::UnsupportedTransferEncoding => {
                formatter.write_str("transfer-encoding is outside the fixture scope")
            }
            Self::DuplicateContentLength => formatter.write_str("duplicate content-length refused"),
            Self::MissingTerminal => formatter.write_str("provider script has no terminal action"),
            Self::WorkerPanic => formatter.write_str("fake provider worker panicked"),
        }
    }
}

impl Error for ProviderError {}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    use crate::conformance::parse_fixture;

    use super::{FakeHttpProvider, ProviderTerminal};

    const VALID: &[u8] =
        include_bytes!("../../../fixtures/conformance/v1/fragmented-completion.json");

    #[test]
    fn fixture_drives_one_real_loopback_http_exchange() {
        let fixture = parse_fixture(VALID).expect("fixture is valid");
        let provider = FakeHttpProvider::spawn(fixture.clone()).expect("provider binds");
        let mut stream = TcpStream::connect(provider.address()).expect("client connects");
        let request = format!(
            "POST /v1/responses HTTP/1.1\r\nhost: localhost\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{}",
            fixture.request().body().len(),
            fixture.request().body()
        );
        stream
            .write_all(request.as_bytes())
            .expect("request writes");
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("response is UTF-8");
        let report = provider.wait().expect("provider completes");

        assert!(response.starts_with("HTTP/1.1 200 Fixture\r\n"));
        assert!(response.contains("data: {\"type\":\"response.output_text.delta\""));
        assert!(report.response_committed);
        assert_eq!(report.fragments_written, 3);
        assert_eq!(report.terminal, ProviderTerminal::Completed);
    }

    #[test]
    fn post_commit_disconnect_remains_partial_and_not_recoverable() {
        let text = String::from_utf8(VALID.to_vec())
            .expect("fixture is UTF-8")
            .replacen("\"kind\": \"complete\"", "\"kind\": \"disconnect\"", 1)
            .replacen(
                "\"terminal_reason\": \"completed\"",
                "\"terminal_reason\": \"upstream_failed\"",
                1,
            )
            .replacen("\"partial\": false", "\"partial\": true", 1);
        let fixture = parse_fixture(text.as_bytes()).expect("disconnect fixture is valid");
        let report = fixture.contract_report().expect("contract is executable");

        assert!(report.committed);
        assert!(report.partial);
        assert!(!report.transparent_recovery_candidate);
    }
}
