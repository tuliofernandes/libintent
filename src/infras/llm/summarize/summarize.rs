use std::sync::Arc;

use serde_json::json;

use crate::infras::llm::backend::LlmBackend;
use crate::intent::Intent;
use crate::types::{ExecutionStatus, IntentInput, IntentResult};

pub struct Summarize {
    backend: Arc<dyn LlmBackend>,
    context: Option<String>,
}

impl Summarize {
    pub fn new(backend: Arc<dyn LlmBackend>, context: Option<String>) -> Self {
        Self { backend, context }
    }
}

impl Intent for Summarize {
    fn name(&self) -> &'static str {
        "summarize"
    }

    fn path(&self) -> &'static str {
        "llm.summarize"
    }

    fn description(&self) -> &'static str {
        "Summarize a block of text using an LLM backend."
    }

    fn execute(&self, input: IntentInput) -> IntentResult {
        let text = match input.data {
            Some(ref d) if !d.trim().is_empty() => d.trim().to_string(),
            _ => {
                return IntentResult {
                    status: ExecutionStatus::Error,
                    result: None,
                    error: Some(
                        "Missing input for llm.summarize: provide text via data".to_string(),
                    ),
                }
            }
        };

        let prompt = build_prompt(self.context.as_deref(), &text);

        match self.backend.generate(&prompt) {
            Ok(response) => IntentResult {
                status: ExecutionStatus::Ok,
                result: Some(json!({ "data": response })),
                error: None,
            },
            Err(e) => IntentResult {
                status: ExecutionStatus::Error,
                result: None,
                error: Some(format!("Failed to execute llm.summarize: {}", e)),
            },
        }
    }
}

fn build_prompt(context: Option<&str>, text: &str) -> String {
    let mut prompt = String::new();
    if let Some(ctx) = context {
        prompt.push_str(ctx);
        prompt.push_str("\n\n");
    }
    prompt.push_str("Summarize the answer for the user input concisely in 1-3 sentences.
Do not add extra context or speculation.
Be direct and factual.\n\n");
    prompt.push_str(text);
    prompt
}
