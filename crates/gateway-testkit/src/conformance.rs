//! Versioned, bounded conformance fixtures for deterministic provider behavior.

use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, Read},
    path::Path,
};

use hafiz_gateway_core::FailureClass;
use serde::Deserialize;

use crate::{Scenario, ScenarioError, ScenarioReport, UpstreamStep};

/// The only fixture schema version understood by this pre-alpha reader.
pub const SCHEMA_VERSION: u16 = 1;
/// Maximum bytes read from one fixture before JSON parsing.
pub const MAX_FIXTURE_BYTES: usize = 256 * 1024;
/// Maximum number of scripted provider steps.
pub const MAX_STEPS: usize = 256;
/// Maximum bytes in one wire fragment.
pub const MAX_FRAGMENT_BYTES: usize = 64 * 1024;
/// Maximum accumulated bytes in one complete protocol event.
pub const MAX_EVENT_BYTES: usize = 64 * 1024;
/// Maximum synthetic request body bytes.
pub const MAX_REQUEST_BODY_BYTES: usize = 64 * 1024;
/// Maximum number of request or response headers.
pub const MAX_HEADERS: usize = 64;
/// Maximum delay allowed in one deterministic step.
pub const MAX_DELAY_MILLISECONDS: u64 = 30_000;

/// A complete synthetic request and deterministic upstream response contract.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceFixture {
    schema_version: u16,
    name: String,
    request: RequestSpec,
    response: ResponseSpec,
    steps: Vec<ProviderStep>,
    expected: ExpectedOutcome,
}

impl ConformanceFixture {
    /// Returns the stable fixture name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the request a fake provider expects.
    #[must_use]
    pub const fn request(&self) -> &RequestSpec {
        &self.request
    }

    /// Returns the response metadata sent by the fake provider.
    #[must_use]
    pub const fn response(&self) -> &ResponseSpec {
        &self.response
    }

    /// Returns the bounded upstream behavior script.
    #[must_use]
    pub fn steps(&self) -> &[ProviderStep] {
        &self.steps
    }

    /// Replays the fixture against the network-neutral stream contract.
    ///
    /// # Errors
    ///
    /// Returns [`FixtureError`] if the fixture is invalid, violates a stream invariant, or its
    /// declared expected outcome does not match the executable contract.
    pub fn contract_report(&self) -> Result<ScenarioReport, FixtureError> {
        let scenario = self.validated_scenario()?;
        let report = scenario.run().map_err(FixtureError::Contract)?;
        self.expected.verify(&report)?;
        Ok(report)
    }

    fn validated_scenario(&self) -> Result<Scenario, FixtureError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(FixtureError::UnsupportedSchema(self.schema_version));
        }
        validate_identifier(&self.name, "name", 96)?;
        self.request.validate()?;
        self.response.validate()?;
        if self.steps.is_empty() || self.steps.len() > MAX_STEPS {
            return Err(FixtureError::InvalidBound("steps"));
        }

        let mut committed = false;
        let mut terminal = false;
        let mut pending_event = Vec::new();
        let mut contract_steps = Vec::with_capacity(self.steps.len());

        for step in &self.steps {
            if terminal {
                return Err(FixtureError::StepAfterTerminal);
            }
            match step {
                ProviderStep::ResponseHeaders => {
                    if committed {
                        return Err(FixtureError::DuplicateCommit);
                    }
                    committed = true;
                    contract_steps.push(UpstreamStep::CommitResponse);
                }
                ProviderStep::Fragment { data, event_end } => {
                    if !committed {
                        return Err(FixtureError::FragmentBeforeCommit);
                    }
                    if data.is_empty() || data.len() > MAX_FRAGMENT_BYTES {
                        return Err(FixtureError::InvalidBound("fragment.data"));
                    }
                    if pending_event.len().saturating_add(data.len()) > MAX_EVENT_BYTES {
                        return Err(FixtureError::InvalidBound("event"));
                    }
                    pending_event.extend_from_slice(data.as_bytes());
                    if *event_end {
                        contract_steps
                            .push(UpstreamStep::EmitEvent(std::mem::take(&mut pending_event)));
                    }
                }
                ProviderStep::Delay { milliseconds } => {
                    if *milliseconds > MAX_DELAY_MILLISECONDS {
                        return Err(FixtureError::InvalidBound("delay.milliseconds"));
                    }
                }
                ProviderStep::Disconnect => {
                    let failure = if committed {
                        FailureClass::StreamPartial
                    } else {
                        FailureClass::NetworkTransient
                    };
                    contract_steps.push(UpstreamStep::Disconnect(failure));
                    terminal = true;
                }
                ProviderStep::Complete => {
                    if !committed {
                        return Err(FixtureError::CompletionBeforeCommit);
                    }
                    if !pending_event.is_empty() {
                        return Err(FixtureError::IncompleteEventAtCompletion);
                    }
                    contract_steps.push(UpstreamStep::Complete);
                    terminal = true;
                }
            }
        }

        if !terminal {
            return Err(FixtureError::MissingTerminal);
        }

        Ok(Scenario::new(
            self.name.clone(),
            MAX_EVENT_BYTES,
            contract_steps,
        ))
    }
}

/// Synthetic request expected by the deterministic provider.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RequestSpec {
    method: String,
    path: String,
    headers: Vec<HeaderField>,
    body: String,
}

impl RequestSpec {
    /// Returns the expected HTTP method.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the expected origin-form request target.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the expected synthetic request body.
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Returns the headers that must be present in the synthetic request.
    #[must_use]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    fn validate(&self) -> Result<(), FixtureError> {
        if self.method.is_empty()
            || self.method.len() > 16
            || !self
                .method
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte == b'-')
        {
            return Err(FixtureError::InvalidRequest("method"));
        }
        if !self.path.starts_with('/')
            || self.path.len() > 2_048
            || self.path.bytes().any(|byte| byte.is_ascii_control())
        {
            return Err(FixtureError::InvalidRequest("path"));
        }
        if self.body.len() > MAX_REQUEST_BODY_BYTES {
            return Err(FixtureError::InvalidBound("request.body"));
        }
        validate_headers(&self.headers, "request.headers")
    }
}

/// Synthetic response metadata used when the fixture commits downstream-visible headers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResponseSpec {
    status: u16,
    headers: Vec<HeaderField>,
}

impl ResponseSpec {
    /// Returns the synthetic response status.
    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    /// Returns the synthetic response headers.
    #[must_use]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    fn validate(&self) -> Result<(), FixtureError> {
        if !(100..=599).contains(&self.status) {
            return Err(FixtureError::InvalidResponse("status"));
        }
        validate_headers(&self.headers, "response.headers")?;
        if self.headers.iter().any(|header| {
            matches!(
                header.name.to_ascii_lowercase().as_str(),
                "connection" | "content-length" | "transfer-encoding"
            )
        }) {
            return Err(FixtureError::InvalidResponse("framing_header"));
        }
        Ok(())
    }
}

/// One bounded, sanitized HTTP header.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HeaderField {
    name: String,
    value: String,
}

impl HeaderField {
    /// Returns the header name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the header value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// One deterministic provider action.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProviderStep {
    /// Write the configured HTTP response status and headers.
    ResponseHeaders,
    /// Write one wire fragment; `event_end` marks a complete protocol event boundary.
    Fragment {
        /// Sanitized UTF-8 bytes written to the socket.
        data: String,
        /// Whether this fragment completes one logical protocol event.
        event_end: bool,
    },
    /// Pause for a bounded duration.
    Delay {
        /// Delay duration in milliseconds.
        milliseconds: u64,
    },
    /// Close the connection without a clean terminal response.
    Disconnect,
    /// Finish the response cleanly.
    Complete,
}

/// Declared observable result checked against the executable domain contract.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct ExpectedOutcome {
    terminal_reason: String,
    committed: bool,
    forwarded_events: u64,
    partial: bool,
    transparent_recovery_candidate: bool,
}

impl ExpectedOutcome {
    fn verify(&self, report: &ScenarioReport) -> Result<(), FixtureError> {
        let checks = [
            (
                self.terminal_reason == report.terminal_reason.code(),
                "expected.terminal_reason",
            ),
            (self.committed == report.committed, "expected.committed"),
            (
                self.forwarded_events == report.forwarded_events,
                "expected.forwarded_events",
            ),
            (self.partial == report.partial, "expected.partial"),
            (
                self.transparent_recovery_candidate == report.transparent_recovery_candidate,
                "expected.transparent_recovery_candidate",
            ),
        ];
        checks
            .into_iter()
            .find_map(|(matches, field)| (!matches).then_some(field))
            .map_or(Ok(()), |field| Err(FixtureError::ExpectedMismatch(field)))
    }
}

/// Loads, parses, and validates a fixture with a hard pre-parse byte bound.
///
/// # Errors
///
/// Returns [`FixtureError`] for I/O failure, over-limit input, invalid JSON, invalid schema, or a
/// declared result that disagrees with the executable contract.
pub fn load_fixture(path: impl AsRef<Path>) -> Result<ConformanceFixture, FixtureError> {
    let file = File::open(path).map_err(|error| FixtureError::Io(error.kind()))?;
    let mut bytes = Vec::with_capacity(MAX_FIXTURE_BYTES.min(8 * 1024));
    file.take((MAX_FIXTURE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| FixtureError::Io(error.kind()))?;
    parse_fixture(&bytes)
}

/// Parses and validates a fixture already held in memory.
///
/// # Errors
///
/// Returns [`FixtureError`] for over-limit input, invalid JSON, invalid schema, or a declared
/// result that disagrees with the executable contract.
pub fn parse_fixture(bytes: &[u8]) -> Result<ConformanceFixture, FixtureError> {
    if bytes.len() > MAX_FIXTURE_BYTES {
        return Err(FixtureError::FixtureTooLarge);
    }
    let fixture: ConformanceFixture =
        serde_json::from_slice(bytes).map_err(|error| FixtureError::Json(error.to_string()))?;
    fixture.contract_report()?;
    Ok(fixture)
}

fn validate_identifier(
    value: &str,
    field: &'static str,
    maximum: usize,
) -> Result<(), FixtureError> {
    if value.is_empty()
        || value.len() > maximum
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(FixtureError::InvalidIdentifier(field));
    }
    Ok(())
}

fn validate_headers(headers: &[HeaderField], field: &'static str) -> Result<(), FixtureError> {
    if headers.len() > MAX_HEADERS {
        return Err(FixtureError::InvalidBound(field));
    }
    for header in headers {
        if header.name.is_empty()
            || header.name.len() > 128
            || !header
                .name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return Err(FixtureError::InvalidHeader("name"));
        }
        if header.value.len() > 8 * 1024
            || header
                .value
                .bytes()
                .any(|byte| matches!(byte, b'\r' | b'\n' | 0))
        {
            return Err(FixtureError::InvalidHeader("value"));
        }
    }
    Ok(())
}

/// A stable validation failure that never includes fixture content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixtureError {
    /// The file could not be read.
    Io(io::ErrorKind),
    /// The pre-parse fixture byte limit was exceeded.
    FixtureTooLarge,
    /// JSON syntax or structure was invalid.
    Json(String),
    /// The schema version is not supported by this reader.
    UnsupportedSchema(u16),
    /// A configured collection or byte length violated a hard bound.
    InvalidBound(&'static str),
    /// A stable identifier was malformed.
    InvalidIdentifier(&'static str),
    /// Synthetic request metadata was malformed.
    InvalidRequest(&'static str),
    /// Synthetic response metadata was malformed.
    InvalidResponse(&'static str),
    /// A header could not be emitted safely.
    InvalidHeader(&'static str),
    /// Response commitment was scripted more than once.
    DuplicateCommit,
    /// Wire bytes were scripted before response commitment.
    FragmentBeforeCommit,
    /// Clean completion was scripted before response commitment.
    CompletionBeforeCommit,
    /// Clean completion left an unterminated event.
    IncompleteEventAtCompletion,
    /// A step followed a terminal action.
    StepAfterTerminal,
    /// The fixture supplied no terminal action.
    MissingTerminal,
    /// The executable stream contract rejected the scenario.
    Contract(ScenarioError),
    /// The declared outcome disagreed with the executable contract.
    ExpectedMismatch(&'static str),
}

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(kind) => write!(formatter, "fixture I/O failed: {kind:?}"),
            Self::FixtureTooLarge => formatter.write_str("fixture exceeds the byte limit"),
            Self::Json(error) => write!(formatter, "invalid fixture JSON: {error}"),
            Self::UnsupportedSchema(version) => {
                write!(formatter, "unsupported fixture schema version: {version}")
            }
            Self::InvalidBound(field) => write!(formatter, "fixture bound violated: {field}"),
            Self::InvalidIdentifier(field) => write!(formatter, "invalid identifier: {field}"),
            Self::InvalidRequest(field) => write!(formatter, "invalid request field: {field}"),
            Self::InvalidResponse(field) => write!(formatter, "invalid response field: {field}"),
            Self::InvalidHeader(field) => write!(formatter, "invalid header field: {field}"),
            Self::DuplicateCommit => {
                formatter.write_str("response headers committed more than once")
            }
            Self::FragmentBeforeCommit => {
                formatter.write_str("fragment emitted before response headers")
            }
            Self::CompletionBeforeCommit => {
                formatter.write_str("response completed before headers")
            }
            Self::IncompleteEventAtCompletion => {
                formatter.write_str("clean completion contains an unfinished event")
            }
            Self::StepAfterTerminal => formatter.write_str("provider step follows terminal action"),
            Self::MissingTerminal => formatter.write_str("fixture has no terminal action"),
            Self::Contract(error) => write!(formatter, "fixture violates stream contract: {error}"),
            Self::ExpectedMismatch(field) => {
                write!(
                    formatter,
                    "declared outcome does not match contract: {field}"
                )
            }
        }
    }
}

impl Error for FixtureError {}

#[cfg(test)]
mod tests {
    use super::{FixtureError, MAX_FIXTURE_BYTES, parse_fixture};

    const VALID: &[u8] =
        include_bytes!("../../../fixtures/conformance/v1/fragmented-completion.json");

    #[test]
    fn fragmented_fixture_matches_contract() {
        let fixture = parse_fixture(VALID).expect("fixture is valid");
        let report = fixture.contract_report().expect("contract is executable");

        assert_eq!(fixture.name(), "fragmented_sse_completion");
        assert!(report.committed);
        assert_eq!(report.forwarded_events, 1);
        assert!(!report.partial);
    }

    #[test]
    fn unsupported_schema_fails_deterministically() {
        let invalid = String::from_utf8(VALID.to_vec())
            .expect("fixture is UTF-8")
            .replacen("\"schema_version\": 1", "\"schema_version\": 2", 1);

        assert_eq!(
            parse_fixture(invalid.as_bytes()),
            Err(FixtureError::UnsupportedSchema(2))
        );
    }

    #[test]
    fn outcome_mismatch_is_rejected() {
        let invalid = String::from_utf8(VALID.to_vec())
            .expect("fixture is UTF-8")
            .replacen("\"forwarded_events\": 1", "\"forwarded_events\": 2", 1);

        assert_eq!(
            parse_fixture(invalid.as_bytes()),
            Err(FixtureError::ExpectedMismatch("expected.forwarded_events"))
        );
    }

    #[test]
    fn over_limit_input_is_refused_before_parsing() {
        let bytes = vec![b' '; MAX_FIXTURE_BYTES + 1];
        assert_eq!(parse_fixture(&bytes), Err(FixtureError::FixtureTooLarge));
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let invalid = String::from_utf8(VALID.to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "\"schema_version\": 1,",
                "\"schema_version\": 1,\n  \"surprise\": true,",
                1,
            );

        assert!(matches!(
            parse_fixture(invalid.as_bytes()),
            Err(FixtureError::Json(_))
        ));
    }
}
