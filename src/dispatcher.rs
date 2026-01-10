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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Executiontatus};

    struct HelloWorld;

    impl Intent for HelloWorld {
        fn name(&self) -> &'static str { "hello_world" }
        fn path(&self) -> &'static str { "core.hello_world" }
        fn description(&self) -> &'static str { "Returns a friendly greeting." }
        fn execute(&self, _input: IntentInput) -> IntentOutput {
            IntentOutput {
                status: Executiontatus::Ok,
                result: Some("Hello, world!".to_string()),
            }
        }
    }

    #[test]
    fn dispatch_should_return_intent_output_with_an_error_if_intent_does_not_exist() {

        let input = IntentInput {
            data: None,
            args: vec![],
        };
        let expected_output = IntentOutput {
            status: Executiontatus::Error,
            result: Some(format!("Intent 'fake.intent' not found")),
        };
        let dispatcher = Dispatcher::new();

        let result = dispatcher.dispatch("fake.intent", input);

        assert_eq!(result, expected_output);

    }

    #[test]
    fn register_should_correctly_register_intent() {

        let mut dispatcher = Dispatcher::new();

        dispatcher.register(Box::new(HelloWorld));

        assert_eq!(dispatcher.intents.len(), 1);
        assert!(dispatcher.intents.contains_key("core.hello_world"));

    }

    #[test]
    fn dispatch_should_return_error_when_intent_returns_error() {
        let input = IntentInput { data: None, args: vec![] };

        struct ErrorIntent;

        impl Intent for ErrorIntent {
            fn name(&self) -> &'static str { "error" }
            fn path(&self) -> &'static str { "core.error" }
            fn description(&self) -> &'static str { "Always fails" }
            fn execute(&self, _input: IntentInput) -> IntentOutput {
                IntentOutput {
                    status: Executiontatus::Error,
                    result: Some("simulated failure".to_string()),
                }
            }
        }

        let mut dispatcher = Dispatcher::new();
        dispatcher.register(Box::new(ErrorIntent));

        let expected = IntentOutput {
            status: Executiontatus::Error,
            result: Some("simulated failure".to_string()),
        };

        let result = dispatcher.dispatch("core.error", input);

        assert_eq!(result, expected);
    }

    #[test]
    fn dispatch_should_return_the_intent_result() {

        let input = IntentInput {
            data: None,
            args: vec![],
        };
        let expected_output = IntentOutput {
            status: Executiontatus::Ok,
            result: Some(String::from("Hello, world!")),
        };
        let mut dispatcher = Dispatcher::new();

        dispatcher.register(Box::new(HelloWorld));

        let result = dispatcher.dispatch("core.hello_world", input);

        assert_eq!(result, expected_output);

    }
}
