---
name: libintent project overview
description: Purpose, architecture, and implemented intents for the libintent Rust library
type: project
---

libintent is a Rust library providing a plugin-style intent dispatch system — a registry of named "intents" callable by dot-notation path (e.g., `fs.list`, `web.search`).

**Why:** Designed as a foundation for an AI agent tool layer. Intents are named capabilities that an agent/orchestrator can register and call by path, with JSON-serializable results.

**How to apply:** When adding new capabilities, implement the `Intent` trait and register via `Dispatcher`. Follow the dot-namespace path convention. Keep `IntentInput` (data + args) and `IntentResult` (status/result/error) as the standard I/O contract.

## Core modules
- `src/intent.rs` — `Intent` trait: name(), path(), description(), execute(IntentInput) -> IntentResult
- `src/dispatcher.rs` — HashMap registry, thread-safe (Send + Sync), dispatches by path string
- `src/types.rs` — IntentInput, IntentResult (with to_json()), CoreError, ExecutionStatus

## Implemented intents
- `fs.list` → src/infras/fs/list/list.rs — lists files/dirs at a path, returns { files, dirs } JSON
- `web.search` → src/infras/web/search/search.rs — searches Brave, fetches up to 5 pages, returns Markdown summary