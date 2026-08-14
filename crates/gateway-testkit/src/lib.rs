//! Deterministic, network-free provider scenarios for contract tests and CLI demonstrations.

#![forbid(unsafe_code)]

use std::{error::Error, fmt};

use hafiz_gateway_core::{
    FailureClass, StreamSignal, StreamState, TerminalReason, TransitionError,
};

/// One scripted behavior from a deterministic upstream provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpstreamStep {
    /// Commit response headers downstream.
    CommitResponse,
    /// Emit one complete protocol event.
    EmitEvent(Vec<u8>),
    /// Disconnect with the classified failure.
    Disconnect(FailureClass),
    /// Complete the stream cleanly.
    Complete,
}

/// A bounded upstream behavior script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scenario {
    name: String,
    max_event_bytes: usize,
    steps: Vec<UpstreamStep>,
}

impl Scenario {
    /// Constructs a named, bounded scenario.
    #[must_use]
    pub fn new(name: impl Into<String>, max_event_bytes: usize, steps: Vec<UpstreamStep>) -> Self {
        Self {
            name: name.into(),
            max_event_bytes,
            steps,
        }
    }

    /// Runs the script against the shared stream contract.
    ///
    /// # Errors
    ///
    /// Returns [`ScenarioError`] when an event exceeds the configured bound, the script contains an
    /// invalid transition, or no terminal step is supplied.
    pub fn run(&self) -> Result<ScenarioReport, ScenarioError> {
        let mut state = ready_for_response().map_err(ScenarioError::Transition)?;
        let mut recovery_candidate = false;

        for step in &self.steps {
            match step {
                UpstreamStep::CommitResponse => state
                    .apply(StreamSignal::CommitResponse)
                    .map_err(ScenarioError::Transition)?,
                UpstreamStep::EmitEvent(event) => {
                    if event.len() > self.max_event_bytes {
                        return Err(ScenarioError::EventTooLarge {
                            actual: event.len(),
                            limit: self.max_event_bytes,
                        });
                    }
                    state
                        .apply(StreamSignal::ForwardEvent)
                        .map_err(ScenarioError::Transition)?;
                }
                UpstreamStep::Disconnect(failure) => {
                    recovery_candidate =
                        !state.downstream_committed() && failure.retryable_before_commit();
                    state
                        .apply(StreamSignal::Terminate(TerminalReason::UpstreamFailed))
                        .map_err(ScenarioError::Transition)?;
                }
                UpstreamStep::Complete => state
                    .apply(StreamSignal::Terminate(TerminalReason::Completed))
                    .map_err(ScenarioError::Transition)?,
            }
        }

        let terminal_reason = state
            .terminal_reason()
            .ok_or(ScenarioError::MissingTerminal)?;
        Ok(ScenarioReport {
            name: self.name.clone(),
            terminal_reason,
            committed: state.downstream_committed(),
            forwarded_events: state.forwarded_events(),
            partial: state.downstream_committed()
                && state.forwarded_events() > 0
                && terminal_reason != TerminalReason::Completed,
            transparent_recovery_candidate: recovery_candidate,
        })
    }
}

/// Observable, non-sensitive result of a deterministic scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioReport {
    /// Scenario identifier.
    pub name: String,
    /// Stable terminal disposition.
    pub terminal_reason: TerminalReason,
    /// Whether downstream commitment occurred.
    pub committed: bool,
    /// Number of complete events forwarded.
    pub forwarded_events: u64,
    /// Whether visible event output ended unsuccessfully.
    pub partial: bool,
    /// Whether the failure remains a candidate for separately budgeted recovery.
    pub transparent_recovery_candidate: bool,
}

/// Error returned by a malformed or incomplete deterministic scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioError {
    /// A scripted event exceeded the scenario bound.
    EventTooLarge {
        /// Observed byte length.
        actual: usize,
        /// Configured byte limit.
        limit: usize,
    },
    /// A step violated the shared stream state machine.
    Transition(TransitionError),
    /// The script ended without a terminal step.
    MissingTerminal,
}

impl fmt::Display for ScenarioError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventTooLarge { actual, limit } => {
                write!(formatter, "event size {actual} exceeds limit {limit}")
            }
            Self::Transition(error) => error.fmt(formatter),
            Self::MissingTerminal => formatter.write_str("scenario has no terminal step"),
        }
    }
}

impl Error for ScenarioError {}

/// Returns one of the built-in contract demonstration scenarios.
#[must_use]
pub fn named_scenario(name: &str) -> Option<Scenario> {
    const EVENT: &[u8] = b"data: {\"type\":\"response.output_text.delta\"}\n\n";
    let steps = match name {
        "complete" => vec![
            UpstreamStep::CommitResponse,
            UpstreamStep::EmitEvent(EVENT.to_vec()),
            UpstreamStep::Complete,
        ],
        "precommit-disconnect" => vec![UpstreamStep::Disconnect(FailureClass::NetworkTransient)],
        "partial-disconnect" => vec![
            UpstreamStep::CommitResponse,
            UpstreamStep::EmitEvent(EVENT.to_vec()),
            UpstreamStep::Disconnect(FailureClass::StreamPartial),
        ],
        _ => return None,
    };

    Some(Scenario::new(name, 64 * 1024, steps))
}

fn ready_for_response() -> Result<StreamState, TransitionError> {
    let mut state = StreamState::new();
    for signal in [
        StreamSignal::Authenticate,
        StreamSignal::AllowPolicy,
        StreamSignal::SelectTarget,
        StreamSignal::BeginUpstreamConnect,
        StreamSignal::AwaitResponseHeaders,
    ] {
        state.apply(signal)?;
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use hafiz_gateway_core::TerminalReason;

    use super::{Scenario, ScenarioError, UpstreamStep, named_scenario};

    #[test]
    fn clean_scenario_completes() {
        let report = named_scenario("complete")
            .expect("scenario exists")
            .run()
            .expect("scenario is valid");

        assert_eq!(report.terminal_reason, TerminalReason::Completed);
        assert!(report.committed);
        assert!(!report.partial);
        assert!(!report.transparent_recovery_candidate);
    }

    #[test]
    fn precommit_disconnect_is_recovery_candidate() {
        let report = named_scenario("precommit-disconnect")
            .expect("scenario exists")
            .run()
            .expect("scenario is valid");

        assert!(!report.committed);
        assert!(report.transparent_recovery_candidate);
    }

    #[test]
    fn partial_disconnect_is_never_transparently_recovered() {
        let report = named_scenario("partial-disconnect")
            .expect("scenario exists")
            .run()
            .expect("scenario is valid");

        assert!(report.committed);
        assert!(report.partial);
        assert!(!report.transparent_recovery_candidate);
    }

    #[test]
    fn event_bound_is_enforced() {
        let scenario = Scenario::new(
            "oversized",
            2,
            vec![
                UpstreamStep::CommitResponse,
                UpstreamStep::EmitEvent(vec![0; 3]),
            ],
        );

        assert_eq!(
            scenario.run(),
            Err(ScenarioError::EventTooLarge {
                actual: 3,
                limit: 2
            })
        );
    }
}
