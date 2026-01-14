use libintent::types::{ExecutionStatus, IntentResult};
use libintent::{infras::fs::list::list::List};
use libintent::{intent::Intent};
use libintent::{types::{IntentInput}};

#[test]
fn should_return_error_if_directory_does_not_exist() {
    let input = IntentInput { data: Some(String::from("/fake/dir")), args: vec![] };
    let result = List.execute(input);
    let expected = IntentResult {
        status: ExecutionStatus::Error,
        result: Some(format!("Failed to read directory: {}", "No such file or directory (os error 2)")),
    };

    assert_eq!(result, expected);
}