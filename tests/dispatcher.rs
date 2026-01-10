use libintent::dispatcher::Dispatcher;
use libintent::intent::Intent;
use libintent::types::{IntentInput, IntentOutput, Executiontatus};

struct HelloWorld;

impl Intent for HelloWorld {
    fn name(&self) -> &'static str { "hello" }
    fn path(&self) -> &'static str { "core.hello_world" }
    fn description(&self) -> &'static str { "Returns a friendly greeting." }
    fn execute(&self, _input: IntentInput) -> IntentOutput {
        IntentOutput { status: Executiontatus::Ok, result: Some("Hello, world!".to_string()) }
    }
}

struct ErrorIntent;

impl Intent for ErrorIntent {
    fn name(&self) -> &'static str { "error" }
    fn path(&self) -> &'static str { "core.error" }
    fn description(&self) -> &'static str { "Always fails" }
    fn execute(&self, _input: IntentInput) -> IntentOutput {
        IntentOutput { status: Executiontatus::Error, result: Some("simulated failure".to_string()) }
    }
}

#[test]
fn dispatch_should_return_intent_output_with_an_error_if_intent_does_not_exist() {
    let input = IntentInput { data: None, args: vec![] };
    let dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch("fake.intent", input);
    let expected = IntentOutput {
        status: Executiontatus::Error,
        result: Some("Intent 'fake.intent' not found".to_string())
    };

    assert_eq!(result, expected);
}

#[test]
fn register_should_correctly_register_intent() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(HelloWorld));
    // since `intents` is private, verify registration by dispatching the path
    let result = dispatcher.dispatch("core.hello_world", input);
    let expected = IntentOutput {
        status: Executiontatus::Ok,
        result: Some("Hello, world!".to_string())
    };

    assert_eq!(result, expected);
}

#[test]
fn dispatch_should_return_the_intent_result() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(HelloWorld));

    let expected = IntentOutput {
        status: Executiontatus::Ok,
        result: Some(String::from("Hello, world!"))
    };
    let result = dispatcher.dispatch("core.hello_world", input);

    assert_eq!(result, expected);
}

#[test]
fn dispatch_should_return_error_when_intent_returns_error() {
    let input = IntentInput { data: None, args: vec![] };
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(Box::new(ErrorIntent));

    let expected = IntentOutput {
        status: Executiontatus::Error,
        result: Some(String::from("simulated failure"))
    };
    let result = dispatcher.dispatch("core.error", input);

    assert_eq!(result, expected);
}
