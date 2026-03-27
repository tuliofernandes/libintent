pub trait LlmBackend: Send + Sync {
    fn generate(&self, prompt: &str) -> Result<String, String>;
}
