use std::collections::HashMap;
use crate::intent::Intent;
use crate::types::{IntentInput, IntentOutput, Executiontatus};

pub struct Dispatcher {
    intents: HashMap<String, Box<dyn Intent>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            intents: HashMap::new(),
        }
    }

    pub fn register(&mut self, intent: Box<dyn Intent>) {
        self.intents.insert(intent.path().to_string(), intent);
    }

    pub fn dispatch(&self, path: &str, input: IntentInput) -> IntentOutput {
        match self.intents.get(path) {
            Some(intent) => intent.execute(input),
            None => IntentOutput {
                status: Executiontatus::Error,
                result: Some(format!("Intent '{}' not found", path)),
            },
        }
    }
}
