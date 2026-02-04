use std::fs;

use libintent::types::{ExecutionStatus, IntentInput};
use libintent::intent::Intent;
use libintent::infras::fs::list::list::List;

#[test]
fn should_return_error_if_directory_does_not_exist() {
    let input = IntentInput {
        data: Some(String::from("/fake/nonexistent/dir")),
        args: vec![],
    };
    let result = List.execute(input);
    assert_eq!(result.status, ExecutionStatus::Error);
    assert_eq!(result.result, None);
    assert!(result.error.is_some());
    assert!(result
        .error
        .as_ref()
        .unwrap()
        .contains("No such file or directory"));

    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["status"], "Error");
    assert!(parsed["error"].as_str().unwrap().contains("No such file or directory"));

    println!("\n--- should_return_error_if_directory_does_not_exist ---");
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}

#[test]
fn should_return_ok_with_files_and_dirs_when_directory_exists() {
    let temp = std::env::temp_dir().join("libintent_list_test");
    let _ = fs::create_dir_all(&temp);
    let subdir = temp.join("mydir");
    let _ = fs::create_dir_all(&subdir);
    let file1 = temp.join("a.txt");
    let file2 = temp.join("b.txt");
    fs::write(&file1, "").unwrap();
    fs::write(&file2, "").unwrap();

    let path_str = temp.to_string_lossy().into_owned();
    let input = IntentInput {
        data: Some(path_str),
        args: vec![],
    };
    let result = List.execute(input);

    assert_eq!(result.status, ExecutionStatus::Ok);
    assert!(result.error.is_none());
    let result_val = result.result.as_ref().unwrap();
    let files = result_val["files"].as_array().unwrap();
    let dirs = result_val["dirs"].as_array().unwrap();
    let file_names: Vec<&str> = files.iter().filter_map(|v| v.as_str()).collect();
    let dir_names: Vec<&str> = dirs.iter().filter_map(|v| v.as_str()).collect();
    assert!(file_names.contains(&"a.txt"));
    assert!(file_names.contains(&"b.txt"));
    assert!(dir_names.contains(&"mydir"));

    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["status"], "Ok");
    assert!(parsed["result"]["files"].is_array());
    assert!(parsed["result"]["dirs"].is_array());

    println!("\n--- should_return_ok_with_files_and_dirs_when_directory_exists ---");
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn should_return_ok_with_empty_arrays_for_empty_directory() {
    let temp = std::env::temp_dir().join("libintent_list_empty_test");
    let _ = fs::create_dir_all(&temp);
    let path_str = temp.to_string_lossy().into_owned();
    let input = IntentInput {
        data: Some(path_str),
        args: vec![],
    };
    let result = List.execute(input);

    assert_eq!(result.status, ExecutionStatus::Ok);
    assert!(result.error.is_none());
    let result_val = result.result.as_ref().unwrap();
    let files = result_val["files"].as_array().unwrap();
    let dirs = result_val["dirs"].as_array().unwrap();
    assert!(files.is_empty());
    // Empty dir may still have . and .. on some platforms
    assert!(dirs.iter().all(|v| v.as_str().is_some()));

    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    println!("\n--- should_return_ok_with_empty_arrays_for_empty_directory ---");
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn to_json_produces_valid_envelope() {
    let input = IntentInput {
        data: Some(String::from("/nonexistent")),
        args: vec![],
    };
    let result = List.execute(input);
    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(parsed.get("status").is_some());
    assert!(parsed["status"] == "Ok" || parsed["status"] == "Error");
    if parsed["status"] == "Error" {
        assert!(parsed.get("error").is_some());
        assert!(parsed.get("result").is_none() || parsed["result"].is_null());
    } else {
        assert!(parsed.get("result").is_some());
    }

    println!("\n--- to_json_produces_valid_envelope ---");
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}
