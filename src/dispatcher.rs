use std::collections::HashMap;

use crate::intent::Intent;
use crate::types::{IntentInput, IntentResult, CoreError};

/// Routes incoming requests to the appropriate registered intent.
///
/// The dispatcher owns a registry of intents keyed by their dot-separated path.
/// Intents are registered once at startup; [`dispatch`](Dispatcher::dispatch) is
/// then called for each incoming request.
///
/// # Example
///
/// ```rust
/// use libintent::dispatcher::Dispatcher;
/// use libintent::intent::Intent;
/// use libintent::types::{ExecutionStatus, IntentInput, IntentResult};
///
/// struct Ping;
/// impl Intent for Ping {
///     fn name(&self) -> &'static str { "ping" }
///     fn path(&self) -> &'static str { "core.ping" }
///     fn description(&self) -> &'static str { "Always succeeds." }
///     fn execute(&self, _input: IntentInput) -> IntentResult {
///         IntentResult { status: ExecutionStatus::Ok, result: None, error: None }
///     }
/// }
///
/// let mut d = Dispatcher::new();
/// d.register(Box::new(Ping)).unwrap();
///
/// let result = d.dispatch("core.ping", IntentInput { data: None, args: vec![] }).unwrap();
/// assert_eq!(result.status, ExecutionStatus::Ok);
/// ```
pub struct Dispatcher {
    intents: HashMap<String, Box<dyn Intent + Send + Sync>>,
}

impl Dispatcher {
    /// Creates an empty dispatcher with no registered intents.
    pub fn new() -> Self {
        Self {
            intents: HashMap::new(),
        }
    }

    /// Registers an intent under its [`path`](crate::intent::Intent::path).
    ///
    /// Returns `Err` if an intent with the same path is already registered.
    pub fn register(&mut self, intent: Box<dyn Intent + Send + Sync>) -> Result<(), String> {
        let path = intent.path().to_string();
        if self.intents.contains_key(&path) {
            Err(format!("Intent '{}' already registered", path))
        } else {
            self.intents.insert(path, intent);
            Ok(())
        }
    }

    /// Dispatches a request to the intent registered at `path`.
    ///
    /// Returns [`CoreError::IntentNotFound`](crate::types::CoreError::IntentNotFound)
    /// if no intent is registered under `path`. Intent-level errors are returned
    /// inside [`IntentResult`](crate::types::IntentResult) with
    /// [`ExecutionStatus::Error`](crate::types::ExecutionStatus::Error), not as `Err`.
    pub fn dispatch(&self, path: &str, input: IntentInput) -> Result<IntentResult, CoreError> {
        let intent = self.intents.get(path)
            .ok_or_else(|| CoreError::IntentNotFound { path: path.to_string() })?;

        Ok(intent.execute(input))
    }
}
