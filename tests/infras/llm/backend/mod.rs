use libintent::infras::llm::backend::{LlmBackend, OllamaBackend};

struct AlwaysOk;

impl LlmBackend for AlwaysOk {
    fn generate(&self, _prompt: &str) -> Result<String, String> {
        Ok("ok".to_string())
    }
}

struct AlwaysErr;

impl LlmBackend for AlwaysErr {
    fn generate(&self, _prompt: &str) -> Result<String, String> {
        Err("backend error".to_string())
    }
}

#[test]
fn backend_generate_returns_ok() {
    let backend = AlwaysOk;
    assert!(backend.generate("hello").is_ok());
}

#[test]
fn backend_generate_returns_err() {
    let backend = AlwaysErr;
    assert!(backend.generate("hello").is_err());
}

#[test]
fn ollama_backend_returns_error_when_server_unreachable() {
    let backend = OllamaBackend::new("http://127.0.0.1:1", "llama3");
    let result = backend.generate("Hello");
    assert!(result.is_err());
}

/// Manual test: requires a live Ollama server at localhost:11434 with a loaded model.
/// Run with: cargo test manual_ollama -- --ignored
#[test]
#[ignore]
fn manual_ollama_generate_returns_ok_against_live_server() {
    let backend = OllamaBackend::new("http://localhost:11434", "llama3");
    let result = backend.generate("Reply with a single word: hello");
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    assert!(!result.unwrap().trim().is_empty());
}
