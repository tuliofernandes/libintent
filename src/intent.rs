use crate::types::{IntentInput, IntentOutput};

pub trait Intent: Send + Sync {
    // fn new(&self, name: String) -> Self;
    fn name(&self) -> &'static str;
    fn path(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, input: IntentInput) -> IntentOutput;
}