// #[derive(Debug, Clone)]
// pub struct IntentContext {
//     // pub user: String,
//     // pub timestamp: DateTime<Utc>,
//     // pub trace_id: Uuid,
//     pub pretend: bool,
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Executiontatus {
    Ok,
    Error,
}

#[derive(Debug, Clone)]
pub struct ExecutionOutput {
    pub status: Executiontatus
}

#[derive(Debug, Clone)]
pub struct IntentInput {
    pub data: Option<String>,
    pub args: Vec<String>,
    // pub context: IntentContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntentOutput {
    pub status: Executiontatus,
    pub result: Option<String>,
}

