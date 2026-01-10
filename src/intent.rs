use crate::types::{IntentInput, IntentResult};

pub trait Intent: Send + Sync {
    fn name(&self) -> &'static str;
    fn path(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, input: IntentInput) -> IntentResult;
}
