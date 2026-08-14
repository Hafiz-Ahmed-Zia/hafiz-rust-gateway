//! Network-stack-neutral contracts for Hafiz Rust Gateway.
//!
//! This crate intentionally contains no HTTP runtime. It makes commitment, failure, and recovery
//! decisions independently testable before the project selects `hyper`/Tokio or Pingora.

#![forbid(unsafe_code)]

pub mod failure;
pub mod retry;
pub mod stream;

pub use failure::{FailureClass, ParseFailureClassError, TerminalReason};
pub use retry::{
    AttemptAction, AttemptBudget, AttemptContext, AttemptDecision, BudgetError, RefusalReason,
};
pub use stream::{StreamPhase, StreamSignal, StreamState, TransitionError};
