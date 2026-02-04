use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Ok,
    Error,
}

#[derive(Debug, Clone)]
pub struct IntentInput {
    pub data: Option<String>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntentResult {
    pub status: ExecutionStatus,
    /// Intent-specific payload when status is Ok.
    pub result: Option<Value>,
    /// Error message when status is Error.
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
    /// Serializes this result as JSON for the binary consumer.
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

#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    IntentNotFound { path: String },
}
