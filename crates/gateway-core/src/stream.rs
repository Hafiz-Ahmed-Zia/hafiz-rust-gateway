//! Streaming request state machine and commitment invariant.

use std::{error::Error, fmt};

use crate::TerminalReason;

/// Coarse request phases shared by all future network-stack implementations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamPhase {
    /// Request accepted for processing.
    Accepted,
    /// Caller identity established.
    Authenticated,
    /// Policy allowed the request.
    PolicyAllowed,
    /// An eligible target was selected.
    TargetSelected,
    /// The upstream connection is being established.
    UpstreamConnecting,
    /// The gateway is waiting to commit a response.
    HeadersPending,
    /// Response headers are committed and events may be forwarded.
    Streaming,
    /// The request reached an immutable terminal state.
    Terminal,
}

/// An event that attempts to advance the streaming state machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamSignal {
    /// Authentication succeeded.
    Authenticate,
    /// Policy evaluation succeeded.
    AllowPolicy,
    /// Routing selected an eligible target.
    SelectTarget,
    /// Upstream connection work started.
    BeginUpstreamConnect,
    /// The request was sent and the gateway awaits response headers.
    AwaitResponseHeaders,
    /// Response headers or body bytes became visible downstream.
    CommitResponse,
    /// One protocol event was forwarded downstream.
    ForwardEvent,
    /// Processing ended with the supplied reason.
    Terminate(TerminalReason),
}

impl StreamSignal {
    const fn code(self) -> &'static str {
        match self {
            Self::Authenticate => "authenticate",
            Self::AllowPolicy => "allow_policy",
            Self::SelectTarget => "select_target",
            Self::BeginUpstreamConnect => "begin_upstream_connect",
            Self::AwaitResponseHeaders => "await_response_headers",
            Self::CommitResponse => "commit_response",
            Self::ForwardEvent => "forward_event",
            Self::Terminate(_) => "terminate",
        }
    }
}

/// Request-local streaming state pinned for the lifetime of one attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamState {
    phase: StreamPhase,
    committed: bool,
    forwarded_events: u64,
    terminal_reason: Option<TerminalReason>,
}

impl Default for StreamState {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamState {
    /// Creates a newly accepted request state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phase: StreamPhase::Accepted,
            committed: false,
            forwarded_events: 0,
            terminal_reason: None,
        }
    }

    /// Returns the current phase.
    #[must_use]
    pub const fn phase(self) -> StreamPhase {
        self.phase
    }

    /// Returns whether response headers or bytes are visible downstream.
    #[must_use]
    pub const fn downstream_committed(self) -> bool {
        self.committed
    }

    /// Returns the number of protocol events forwarded downstream.
    #[must_use]
    pub const fn forwarded_events(self) -> u64 {
        self.forwarded_events
    }

    /// Returns the terminal reason once the request has ended.
    #[must_use]
    pub const fn terminal_reason(self) -> Option<TerminalReason> {
        self.terminal_reason
    }

    /// Applies one state transition.
    ///
    /// # Errors
    ///
    /// Returns [`TransitionError`] for an invalid or post-terminal transition.
    pub fn apply(&mut self, signal: StreamSignal) -> Result<(), TransitionError> {
        let next_phase = match (self.phase, signal) {
            (StreamPhase::Accepted, StreamSignal::Authenticate) => StreamPhase::Authenticated,
            (StreamPhase::Authenticated, StreamSignal::AllowPolicy) => StreamPhase::PolicyAllowed,
            (StreamPhase::PolicyAllowed, StreamSignal::SelectTarget) => StreamPhase::TargetSelected,
            (StreamPhase::TargetSelected, StreamSignal::BeginUpstreamConnect) => {
                StreamPhase::UpstreamConnecting
            }
            (StreamPhase::UpstreamConnecting, StreamSignal::AwaitResponseHeaders) => {
                StreamPhase::HeadersPending
            }
            (StreamPhase::HeadersPending, StreamSignal::CommitResponse) => {
                self.committed = true;
                StreamPhase::Streaming
            }
            (StreamPhase::Streaming, StreamSignal::ForwardEvent) => {
                self.forwarded_events = self.forwarded_events.saturating_add(1);
                StreamPhase::Streaming
            }
            (StreamPhase::Streaming, StreamSignal::Terminate(TerminalReason::Completed)) => {
                self.terminal_reason = Some(TerminalReason::Completed);
                StreamPhase::Terminal
            }
            (phase, StreamSignal::Terminate(reason))
                if phase != StreamPhase::Terminal && reason != TerminalReason::Completed =>
            {
                self.terminal_reason = Some(reason);
                StreamPhase::Terminal
            }
            _ => return Err(TransitionError::new(self.phase, signal.code())),
        };

        self.phase = next_phase;
        Ok(())
    }
}

/// An invalid state transition without sensitive request data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransitionError {
    from: StreamPhase,
    signal: &'static str,
}

impl TransitionError {
    const fn new(from: StreamPhase, signal: &'static str) -> Self {
        Self { from, signal }
    }

    /// Returns the phase where the transition was refused.
    #[must_use]
    pub const fn from(self) -> StreamPhase {
        self.from
    }

    /// Returns the stable signal code that was refused.
    #[must_use]
    pub const fn signal(self) -> &'static str {
        self.signal
    }
}

impl fmt::Display for TransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid stream transition from {:?} using {}",
            self.from, self.signal
        )
    }
}

impl Error for TransitionError {}

#[cfg(test)]
mod tests {
    use crate::{StreamPhase, StreamSignal, StreamState, TerminalReason};

    fn ready_to_commit() -> StreamState {
        let mut state = StreamState::new();
        for signal in [
            StreamSignal::Authenticate,
            StreamSignal::AllowPolicy,
            StreamSignal::SelectTarget,
            StreamSignal::BeginUpstreamConnect,
            StreamSignal::AwaitResponseHeaders,
        ] {
            state.apply(signal).expect("fixture transition is valid");
        }
        state
    }

    #[test]
    fn happy_path_commits_and_completes() {
        let mut state = ready_to_commit();
        state
            .apply(StreamSignal::CommitResponse)
            .expect("response can commit");
        state
            .apply(StreamSignal::ForwardEvent)
            .expect("event can be forwarded");
        state
            .apply(StreamSignal::Terminate(TerminalReason::Completed))
            .expect("stream can complete");

        assert_eq!(state.phase(), StreamPhase::Terminal);
        assert!(state.downstream_committed());
        assert_eq!(state.forwarded_events(), 1);
        assert_eq!(state.terminal_reason(), Some(TerminalReason::Completed));
    }

    #[test]
    fn completion_before_commit_is_invalid() {
        let mut state = ready_to_commit();
        assert!(
            state
                .apply(StreamSignal::Terminate(TerminalReason::Completed))
                .is_err()
        );
    }

    #[test]
    fn terminal_state_is_immutable() {
        let mut state = ready_to_commit();
        state
            .apply(StreamSignal::Terminate(TerminalReason::UpstreamFailed))
            .expect("failure is terminal");

        assert!(state.apply(StreamSignal::CommitResponse).is_err());
    }
}
