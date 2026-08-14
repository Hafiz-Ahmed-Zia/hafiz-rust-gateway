//! Explicit retry and fallback budget evaluation.

use std::{error::Error, fmt, time::Duration};

use crate::FailureClass;

/// A transparent recovery action requested by the route plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptAction {
    /// Retry the same eligible target.
    RetrySameTarget,
    /// Move to another capability-compatible target.
    Fallback,
}

/// Immutable bounds applied to every transparent recovery decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttemptBudget {
    max_attempts: u16,
    max_retry_elapsed: Duration,
    minimum_remaining: Duration,
}

impl AttemptBudget {
    /// Constructs a validated attempt budget.
    ///
    /// # Errors
    ///
    /// Returns [`BudgetError`] when `max_attempts` is zero.
    pub const fn new(
        max_attempts: u16,
        max_retry_elapsed: Duration,
        minimum_remaining: Duration,
    ) -> Result<Self, BudgetError> {
        if max_attempts == 0 {
            return Err(BudgetError);
        }
        Ok(Self {
            max_attempts,
            max_retry_elapsed,
            minimum_remaining,
        })
    }

    /// Evaluates one proposed retry or fallback without performing it.
    #[must_use]
    pub fn evaluate(self, action: AttemptAction, context: AttemptContext) -> AttemptDecision {
        let refusal = if context.downstream_committed {
            Some(RefusalReason::DownstreamCommitted)
        } else if !context.replay_safe {
            Some(RefusalReason::ReplayUnsafe)
        } else if !context.failure.retryable_before_commit() {
            Some(RefusalReason::FailureNotRetryable)
        } else if context.attempts_started >= self.max_attempts {
            Some(RefusalReason::AttemptBudgetExhausted)
        } else if context.retry_elapsed >= self.max_retry_elapsed {
            Some(RefusalReason::RetryTimeBudgetExhausted)
        } else if context.deadline_remaining < self.minimum_remaining {
            Some(RefusalReason::InsufficientDeadline)
        } else {
            None
        };

        refusal.map_or(AttemptDecision::Permitted(action), AttemptDecision::Refused)
    }
}

/// Request-local facts needed to evaluate transparent recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttemptContext {
    /// Number of attempts already started, including the current attempt.
    pub attempts_started: u16,
    /// Time already consumed by retries or fallbacks.
    pub retry_elapsed: Duration,
    /// Time remaining until the end-to-end deadline.
    pub deadline_remaining: Duration,
    /// Whether response headers or bytes are visible downstream.
    pub downstream_committed: bool,
    /// Whether replay is permitted by the request/provider contract.
    pub replay_safe: bool,
    /// Classified reason the current attempt failed.
    pub failure: FailureClass,
}

/// Outcome of evaluating a proposed retry or fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptDecision {
    /// Recovery is allowed by the local contract and budgets.
    Permitted(AttemptAction),
    /// Recovery is refused for a stable reason.
    Refused(RefusalReason),
}

/// Stable reason a retry or fallback was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefusalReason {
    /// Downstream-visible output makes transparent recovery observable.
    DownstreamCommitted,
    /// The request or provider contract does not permit replay.
    ReplayUnsafe,
    /// The classified failure is not eligible for transparent recovery.
    FailureNotRetryable,
    /// The maximum attempt count has been reached.
    AttemptBudgetExhausted,
    /// The retry-specific time budget has been consumed.
    RetryTimeBudgetExhausted,
    /// Too little end-to-end deadline remains for a useful attempt.
    InsufficientDeadline,
}

impl RefusalReason {
    /// Returns the stable machine-readable refusal code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::DownstreamCommitted => "downstream_committed",
            Self::ReplayUnsafe => "replay_unsafe",
            Self::FailureNotRetryable => "failure_not_retryable",
            Self::AttemptBudgetExhausted => "attempt_budget_exhausted",
            Self::RetryTimeBudgetExhausted => "retry_time_budget_exhausted",
            Self::InsufficientDeadline => "insufficient_deadline",
        }
    }
}

/// Returned when an attempt budget cannot represent a valid policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BudgetError;

impl fmt::Display for BudgetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("max_attempts must be greater than zero")
    }
}

impl Error for BudgetError {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        AttemptAction, AttemptBudget, AttemptContext, AttemptDecision, FailureClass, RefusalReason,
    };

    fn context() -> AttemptContext {
        AttemptContext {
            attempts_started: 1,
            retry_elapsed: Duration::from_millis(10),
            deadline_remaining: Duration::from_secs(2),
            downstream_committed: false,
            replay_safe: true,
            failure: FailureClass::NetworkTransient,
        }
    }

    fn budget() -> AttemptBudget {
        AttemptBudget::new(3, Duration::from_secs(1), Duration::from_millis(100))
            .expect("test budget is valid")
    }

    #[test]
    fn allows_bounded_pre_commit_retry() {
        assert_eq!(
            budget().evaluate(AttemptAction::RetrySameTarget, context()),
            AttemptDecision::Permitted(AttemptAction::RetrySameTarget)
        );
    }

    #[test]
    fn refuses_every_post_commit_recovery() {
        let mut committed = context();
        committed.downstream_committed = true;

        assert_eq!(
            budget().evaluate(AttemptAction::Fallback, committed),
            AttemptDecision::Refused(RefusalReason::DownstreamCommitted)
        );
    }

    #[test]
    fn refuses_non_retryable_failure() {
        let mut invalid = context();
        invalid.failure = FailureClass::Client;

        assert_eq!(
            budget().evaluate(AttemptAction::RetrySameTarget, invalid),
            AttemptDecision::Refused(RefusalReason::FailureNotRetryable)
        );
    }
}
