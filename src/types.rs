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
    pub result: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    IntentNotFound { path: String },
}
