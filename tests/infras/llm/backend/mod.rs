use libintent::infras::llm::backend::LlmBackend;

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
