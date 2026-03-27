mod ollama;

pub use ollama::OllamaBackend;

pub trait LlmBackend: Send + Sync {
    fn generate(&self, prompt: &str) -> Result<String, String>;
}
