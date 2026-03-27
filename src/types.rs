use serde::Serialize;
use serde_json::Value;

/// Indicates whether an intent execution completed successfully.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    /// The intent ran to completion without errors.
    Ok,
    /// The intent encountered an error. See [`IntentResult::error`] for the message.
    Error,
}

/// Input passed to an intent at execution time.
#[derive(Debug, Clone)]
pub struct IntentInput {
    /// Primary payload — typically the main argument the intent operates on
    /// (e.g. a search query or a directory path). `None` when not provided.
    pub data: Option<String>,
    /// Additional flags and positional arguments, in order (e.g. `["--pages", "3"]`).
    pub args: Vec<String>,
}

/// The outcome of a single intent execution.
#[derive(Debug, Clone, PartialEq)]
pub struct IntentResult {
    /// Whether the intent succeeded or failed.
    pub status: ExecutionStatus,
    /// Intent-specific payload, present when `status` is [`ExecutionStatus::Ok`].
    ///
    /// The shape of this value is defined by each intent and documented in its
    /// man page (e.g. `man web.search`).
    pub result: Option<Value>,
    /// Human-readable error message, present when `status` is [`ExecutionStatus::Error`].
    pub error: Option<String>,
}

#[derive(Serialize)]
struct IntentResultEnvelope {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl IntentResult {
    /// Serialises this result to a JSON string for consumption by the host binary.
    ///
    /// The envelope always contains a `"status"` field (`"Ok"` or `"Error"`).
    /// `"result"` and `"error"` are omitted when `None`.
    ///
    /// # Example output (success)
    ///
    /// ```json
    /// {"status":"Ok","result":{"data":{"files":["a.txt"],"dirs":["src"]}}}
    /// ```
    ///
    /// # Example output (error)
    ///
    /// ```json
    /// {"status":"Error","error":"Missing query for web.search intent"}
    /// ```
    pub fn to_json(&self) -> String {
        let status = match self.status {
            ExecutionStatus::Ok => "Ok",
            ExecutionStatus::Error => "Error",
        };
        let envelope = IntentResultEnvelope {
            status,
            result: self.result.clone(),
            error: self.error.clone(),
        };
        serde_json::to_string(&envelope).expect("IntentResult envelope is serializable")
    }
}

/// Errors returned by libintent's core infrastructure.
///
/// These are distinct from intent-level errors, which are encoded inside
/// [`IntentResult`] with [`ExecutionStatus::Error`].
#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    /// No intent is registered under the given path.
    IntentNotFound { path: String },
}
