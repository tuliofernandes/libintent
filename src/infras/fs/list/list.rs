use std::fs;

use serde_json::json;

use crate::intent::Intent;
use crate::types::{ExecutionStatus, IntentInput, IntentResult};

pub struct List;

impl Intent for List {
    fn name(&self) -> &'static str { "list" }
    fn path(&self) -> &'static str { "fs.list" }
    fn description(&self) -> &'static str { "List files in a directory." }
    fn execute(&self, input: IntentInput) -> IntentResult {
        let path = input.data.as_deref().unwrap_or(".");
        match fs::read_dir(path) {
            Ok(entries) => {
                let mut files: Vec<String> = Vec::new();
                let mut dirs: Vec<String> = Vec::new();
                for e in entries.filter_map(|e| e.ok()) {
                    let name = e.file_name().to_string_lossy().into_owned();
                    match e.metadata().map(|m| m.is_dir()) {
                        Ok(true) => dirs.push(name),
                        _ => files.push(name),
                    }
                }
                files.sort();
                dirs.sort();
                IntentResult {
                    status: ExecutionStatus::Ok,
                    result: Some(json!({ "data": { "files": files, "dirs": dirs } })),
                    error: None,
                }
            }
            Err(e) => IntentResult {
                status: ExecutionStatus::Error,
                result: None,
                error: Some(format!("Failed to read directory: {}", e)),
            },
        }
    }
}
