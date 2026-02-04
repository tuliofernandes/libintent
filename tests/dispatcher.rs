use serde_json::json;
use libintent::types::{IntentInput, IntentResult, ExecutionStatus, CoreError};
use libintent::intent::Intent;
use libintent::dispatcher::Dispatcher;

struct HelloWorld;

impl Intent for HelloWorld {
    fn name(&self) -> &'static str { "hello" }
    fn path(&self) -> &'static str { "core.hello_world" }
    fn description(&self) -> &'static str { "Returns a friendly greeting." }
    fn execute(&self, _input: IntentInput) -> IntentResult {
        IntentResult {
            status: ExecutionStatus::Ok,
            result: Some(json!("Hello, world!")),
            error: None,
        }
    }
}

struct ErrorIntent;

impl Intent for ErrorIntent {
    fn name(&self) -> &'static str { "error" }
    fn path(&self) -> &'static str { "core.error" }
    fn description(&self) -> &'static str { "Always fails" }
    fn execute(&self, _input: IntentInput) -> IntentResult {
        IntentResult {
            status: ExecutionStatus::Error,
            result: None,
            error: Some("simulated failure".to_string()),
        }
    }
}

#[test]
fn dispatch_should_return_error_if_intent_does_not_exist() {
    let input = IntentInput { data: None, args: vec![] };
    let dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch("fake.intent", input);
    let expected = Err(CoreError::IntentNotFound { path: "fake.intent".to_string() });

    assert_eq!(result, expected);
}

#[test]
fn register_should_correctly_register_intent() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(HelloWorld)).unwrap();
    
    let result = dispatcher.dispatch("core.hello_world", input);
    let expected = Ok(IntentResult {
        status: ExecutionStatus::Ok,
        result: Some(json!("Hello, world!")),
        error: None,
    });

    assert_eq!(result, expected);
}

#[test]
fn register_should_return_error_when_intent_already_exists() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.register(Box::new(HelloWorld)).unwrap();
    let result = dispatcher.register(Box::new(HelloWorld));
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Intent 'core.hello_world' already registered");
}

#[test]
fn dispatch_should_return_intent_result_ok() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(HelloWorld)).unwrap();

    let expected = Ok(IntentResult {
        status: ExecutionStatus::Ok,
        result: Some(json!("Hello, world!")),
        error: None,
    });
    let result = dispatcher.dispatch("core.hello_world", input);

    assert_eq!(result, expected);
}

#[test]
fn dispatch_should_return_intent_result_error_status() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(ErrorIntent)).unwrap();

    let expected = Ok(IntentResult {
        status: ExecutionStatus::Error,
        result: None,
        error: Some("simulated failure".to_string()),
    });
    let result = dispatcher.dispatch("core.error", input);

    assert_eq!(result, expected);
}
