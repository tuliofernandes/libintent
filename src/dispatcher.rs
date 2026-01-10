use std::collections::HashMap;

use crate::intent::Intent;
use crate::types::{IntentInput, IntentResult, CoreError};

pub struct Dispatcher {
    intents: HashMap<String, Box<dyn Intent + Send + Sync>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            intents: HashMap::new(),
        }
    }

    pub fn register(&mut self, intent: Box<dyn Intent + Send + Sync>) -> Result<(), String> {
        let path = intent.path().to_string();
        if self.intents.contains_key(&path) {
            Err(format!("Intent '{}' already registered", path))
        } else {
            self.intents.insert(path, intent);
            Ok(())
        }
    }

    pub fn dispatch(&self, path: &str, input: IntentInput) -> Result<IntentResult, CoreError> {
        let intent = self.intents.get(path)
            .ok_or_else(|| CoreError::IntentNotFound { path: path.to_string() })?;

        Ok(intent.execute(input))
    }
}
