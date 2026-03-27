use crate::types::{IntentInput, IntentResult};

/// The contract that all libintent intents must satisfy.
///
/// An intent is the smallest unit of executable behaviour in libintent.
/// Each implementation is identified by a dot-separated path (e.g. `"web.search"`)
/// and carries out a single, well-defined action when [`execute`](Intent::execute) is called.
///
/// # Implementing an intent
///
/// ```rust
/// use libintent::intent::Intent;
/// use libintent::types::{ExecutionStatus, IntentInput, IntentResult};
///
/// struct Ping;
///
/// impl Intent for Ping {
///     fn name(&self) -> &'static str { "ping" }
///     fn path(&self) -> &'static str { "core.ping" }
///     fn description(&self) -> &'static str { "Always succeeds." }
///     fn execute(&self, _input: IntentInput) -> IntentResult {
///         IntentResult { status: ExecutionStatus::Ok, result: None, error: None }
///     }
/// }
/// ```
pub trait Intent: Send + Sync {
    /// Short, human-readable name for this intent (e.g. `"search"`).
    ///
    /// Used for display purposes; uniqueness is not enforced by the library.
    fn name(&self) -> &'static str;

    /// Dot-separated path that uniquely identifies this intent (e.g. `"web.search"`).
    ///
    /// This is the key used by [`Dispatcher::register`](crate::dispatcher::Dispatcher::register)
    /// and [`Dispatcher::dispatch`](crate::dispatcher::Dispatcher::dispatch).
    /// Must be unique across all intents registered in a given dispatcher.
    fn path(&self) -> &'static str;

    /// One-line description of what this intent does.
    ///
    /// Surfaced to end-users in help output and error messages.
    fn description(&self) -> &'static str;

    /// Executes the intent and returns a result.
    ///
    /// Implementations should never panic — all errors must be encoded in the returned
    /// [`IntentResult`](crate::types::IntentResult) with
    /// [`ExecutionStatus::Error`](crate::types::ExecutionStatus::Error).
    fn execute(&self, input: IntentInput) -> IntentResult;
}
