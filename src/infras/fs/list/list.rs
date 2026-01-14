use std::fs;

use crate::intent::Intent;
use crate::types::{IntentInput, IntentResult, ExecutionStatus};

pub struct List;

impl Intent for List {
    fn name(&self) -> &'static str { "list" }
    fn path(&self) -> &'static str { "fs.list" }
    fn description(&self) -> &'static str { "List files in a directory." }
    fn execute(&self, _input: IntentInput) -> IntentResult {        
        match fs::read_dir(_input.data.as_deref().unwrap_or(".")) {
            Ok(entries) => {
                let names: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect();

                IntentResult {
                    status: ExecutionStatus::Ok,
                    result: Some(names.join("\n")),
                }
            }
            Err(e) => IntentResult {
                status: ExecutionStatus::Error,
                result: Some(format!("Failed to read directory: {}", e)),
            },
        }
    }
}