//! A library for building intent-based CLI tools.
//!
//! Intents are discrete units of executable behaviour identified by a dot-separated
//! path (e.g. `"web.search"`, `"fs.list"`). A [`Dispatcher`](dispatcher::Dispatcher)
//! routes incoming requests to the right intent at runtime.
//!
//! ## Core model
//!
//! - Implement the [`Intent`](intent::Intent) trait to define a new capability.
//! - Register intents with a [`Dispatcher`](dispatcher::Dispatcher).
//! - Call [`dispatch`](dispatcher::Dispatcher::dispatch) with a path and an
//!   [`IntentInput`](types::IntentInput) to execute.
//! - Read the [`IntentResult`](types::IntentResult) or serialise it to JSON with
//!   [`to_json`](types::IntentResult::to_json).
//!
//! ## Quick start
//!
//! ```rust
//! use libintent::dispatcher::Dispatcher;
//! use libintent::intent::Intent;
//! use libintent::types::{ExecutionStatus, IntentInput, IntentResult};
//!
//! struct Ping;
//!
//! impl Intent for Ping {
//!     fn name(&self) -> &'static str { "ping" }
//!     fn path(&self) -> &'static str { "core.ping" }
//!     fn description(&self) -> &'static str { "Always succeeds." }
//!     fn execute(&self, _input: IntentInput) -> IntentResult {
//!         IntentResult { status: ExecutionStatus::Ok, result: None, error: None }
//!     }
//! }
//!
//! let mut dispatcher = Dispatcher::new();
//! dispatcher.register(Box::new(Ping)).unwrap();
//!
//! let input = IntentInput { data: None, args: vec![] };
//! let result = dispatcher.dispatch("core.ping", input).unwrap();
//! assert_eq!(result.status, ExecutionStatus::Ok);
//! ```

pub mod types;
pub mod intent;
pub mod dispatcher;
pub mod infras;