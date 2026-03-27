use std::sync::{Arc, Mutex};

use libintent::infras::llm::backend::LlmBackend;
use libintent::infras::llm::summarize::Summarize;
use libintent::intent::Intent;
use libintent::types::{ExecutionStatus, IntentInput};

// --- Test doubles ---

struct MockOk;

impl LlmBackend for MockOk {
    fn generate(&self, _prompt: &str) -> Result<String, String> {
        Ok("summarized text".to_string())
    }
}

struct MockErr;

impl LlmBackend for MockErr {
    fn generate(&self, _prompt: &str) -> Result<String, String> {
        Err("model error".to_string())
    }
}

struct CapturingBackend {
    captured: Arc<Mutex<String>>,
}

impl LlmBackend for CapturingBackend {
    fn generate(&self, prompt: &str) -> Result<String, String> {
        *self.captured.lock().unwrap() = prompt.to_string();
        Ok("captured".to_string())
    }
}

// --- Tests ---

#[test]
fn should_return_error_if_input_missing() {
    let intent = Summarize::new(Arc::new(MockOk), None);
    let input = IntentInput { data: None, args: vec![] };

    let result = intent.execute(input);

    assert_eq!(result.status, ExecutionStatus::Error);
    assert!(result.result.is_none());

    let err = result.error.as_deref().unwrap_or("");
    assert!(err.to_lowercase().contains("missing"));
}

#[test]
fn should_return_ok_when_backend_succeeds() {
    let intent = Summarize::new(Arc::new(MockOk), None);
    let input = IntentInput {
        data: Some("Some long text to summarize.".to_string()),
        args: vec![],
    };

    let result = intent.execute(input);

    assert_eq!(result.status, ExecutionStatus::Ok);
    assert!(result.error.is_none());

    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["result"]["data"], "summarized text");
}

#[test]
fn should_return_error_when_backend_fails() {
    let intent = Summarize::new(Arc::new(MockErr), None);
    let input = IntentInput {
        data: Some("Some text.".to_string()),
        args: vec![],
    };

    let result = intent.execute(input);

    assert_eq!(result.status, ExecutionStatus::Error);
    assert!(result.result.is_none());

    let err = result.error.as_deref().unwrap_or("");
    assert!(err.contains("model error"));
}

#[test]
fn should_include_context_in_prompt_when_set() {
    let captured = Arc::new(Mutex::new(String::new()));
    let backend = CapturingBackend { captured: Arc::clone(&captured) };
    let intent = Summarize::new(
        Arc::new(backend),
        Some("You are a formal assistant.".to_string()),
    );
    let input = IntentInput {
        data: Some("Text to summarize.".to_string()),
        args: vec![],
    };

    intent.execute(input);

    let prompt = captured.lock().unwrap().clone();
    assert!(prompt.contains("You are a formal assistant."), "Context must appear in prompt");
    assert!(prompt.contains("Text to summarize."), "User input must appear in prompt");
}
