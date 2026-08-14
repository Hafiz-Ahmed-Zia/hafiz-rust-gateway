//! Stable failure and terminal reason taxonomies.

use std::{error::Error, fmt, str::FromStr};

/// A stable, non-sensitive failure classification used by recovery policy and telemetry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClass {
    /// Invalid caller input or authentication.
    Client,
    /// A local policy rejected the operation.
    Policy,
    /// The gateway cannot safely admit more work.
    Overload,
    /// A transient network failure occurred before commitment.
    NetworkTransient,
    /// The provider asked the caller to reduce or defer traffic.
    ProviderThrottle,
    /// The provider returned an eligible server-side failure.
    ProviderServer,
    /// The provider rejected the request as invalid or unsupported.
    ProviderClient,
    /// The upstream failed after downstream-visible output.
    StreamPartial,
    /// An optional telemetry dependency failed.
    Telemetry,
    /// A configuration snapshot could not be activated.
    Configuration,
}

impl FailureClass {
    /// Returns the stable machine-readable reason code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Client => "client",
            Self::Policy => "policy",
            Self::Overload => "overload",
            Self::NetworkTransient => "network_transient",
            Self::ProviderThrottle => "provider_throttle",
            Self::ProviderServer => "provider_server",
            Self::ProviderClient => "provider_client",
            Self::StreamPartial => "stream_partial",
            Self::Telemetry => "telemetry",
            Self::Configuration => "configuration",
        }
    }

    /// Returns whether the class may be retried before commitment, subject to all other budgets.
    #[must_use]
    pub const fn retryable_before_commit(self) -> bool {
        matches!(
            self,
            Self::NetworkTransient | Self::ProviderThrottle | Self::ProviderServer
        )
    }

    /// Returns whether this failure should contribute to target health state.
    #[must_use]
    pub const fn affects_target_health(self) -> bool {
        matches!(
            self,
            Self::NetworkTransient | Self::ProviderServer | Self::StreamPartial
        )
    }
}

impl fmt::Display for FailureClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl FromStr for FailureClass {
    type Err = ParseFailureClassError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "client" => Ok(Self::Client),
            "policy" => Ok(Self::Policy),
            "overload" => Ok(Self::Overload),
            "network_transient" => Ok(Self::NetworkTransient),
            "provider_throttle" => Ok(Self::ProviderThrottle),
            "provider_server" => Ok(Self::ProviderServer),
            "provider_client" => Ok(Self::ProviderClient),
            "stream_partial" => Ok(Self::StreamPartial),
            "telemetry" => Ok(Self::Telemetry),
            "configuration" => Ok(Self::Configuration),
            _ => Err(ParseFailureClassError),
        }
    }
}

/// Returned when a failure code is not part of the stable taxonomy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseFailureClassError;

impl fmt::Display for ParseFailureClassError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unknown failure class")
    }
}

impl Error for ParseFailureClassError {}

/// A stable terminal disposition for a streaming request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalReason {
    /// The stream completed according to its protocol contract.
    Completed,
    /// The downstream client cancelled or disconnected.
    ClientCancelled,
    /// The upstream failed or returned a malformed partial response.
    UpstreamFailed,
    /// A policy explicitly terminated the request.
    PolicyTerminated,
    /// A request or phase deadline expired.
    DeadlineExceeded,
    /// Load-shedding policy terminated admitted work.
    OverloadTerminated,
    /// The graceful shutdown deadline expired.
    ShutdownTerminated,
}

impl TerminalReason {
    /// Returns the stable machine-readable terminal code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::ClientCancelled => "client_cancelled",
            Self::UpstreamFailed => "upstream_failed",
            Self::PolicyTerminated => "policy_terminated",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::OverloadTerminated => "overload_terminated",
            Self::ShutdownTerminated => "shutdown_terminated",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FailureClass;

    #[test]
    fn only_transient_provider_failures_are_retry_candidates() {
        assert!(FailureClass::NetworkTransient.retryable_before_commit());
        assert!(FailureClass::ProviderThrottle.retryable_before_commit());
        assert!(FailureClass::ProviderServer.retryable_before_commit());
        assert!(!FailureClass::Client.retryable_before_commit());
        assert!(!FailureClass::StreamPartial.retryable_before_commit());
        assert!(!FailureClass::Telemetry.retryable_before_commit());
    }
}
