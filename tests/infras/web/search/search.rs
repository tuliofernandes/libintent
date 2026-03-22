use libintent::intent::Intent;
use libintent::infras::web::search::search::{extract_page_text, parse_brave_results, Search};
use libintent::types::{ExecutionStatus, IntentInput};

#[test]
fn should_return_error_if_query_missing() {
    let input = IntentInput {
        data: None,
        args: vec![],
    };

    let result = Search.execute(input);

    assert_eq!(result.status, ExecutionStatus::Error);
    assert!(result.result.is_none());
    assert!(result.error.as_ref().is_some());

    let out = result.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();

    assert_eq!(parsed["status"], "Error");
    assert!(parsed["error"].as_str().unwrap().to_lowercase().contains("query"));
}

/// This is an integration-style test that depends on live web access.
/// It is marked as ignored so regular `cargo test` runs remain deterministic.
#[test]
#[ignore]
fn should_return_ok_with_markdown_for_simple_query_or_error_on_network_failure() {
    let input = IntentInput {
        data: Some(String::from("Bob Dylan Blood on the Tracks")),
        args: vec![],
    };

    let result = Search.execute(input);

    match result.status {
        ExecutionStatus::Ok => {
            assert!(result.error.is_none());
            let markdown_value = result
                .result
                .as_ref()
                .and_then(|v| v.get("data"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            assert!(!markdown_value.trim().is_empty());
        }
        ExecutionStatus::Error => {
            // On network or scraping errors we still expect a well-formed error.
            assert!(result.result.is_none());
            assert!(result.error.as_ref().is_some());
        }
    }
}

#[test]
fn parse_brave_results_respects_limit() {
    let html = r##"
        <html><body>
            <a href="https://site1.com/article">Site One</a>
            <a href="https://site2.com/article">Site Two</a>
            <a href="https://site3.com/article">Site Three</a>
            <a href="https://site4.com/article">Site Four</a>
            <a href="https://site5.com/article">Site Five</a>
        </body></html>
    "##;

    let results = parse_brave_results(html, 2);

    assert_eq!(results.len(), 2);
}

#[test]
fn parse_brave_results_returns_all_when_limit_exceeds_available() {
    let html = r##"
        <html><body>
            <a href="https://site1.com/article">Site One</a>
            <a href="https://site2.com/article">Site Two</a>
        </body></html>
    "##;

    let results = parse_brave_results(html, 10);

    assert_eq!(results.len(), 2);
}

#[test]
fn parse_brave_results_extracts_unique_http_links() {
    let html = r##"
        <html>
          <body>
            <a href="https://example.com/page1">First result</a>
            <a href="https://example.com/page1">Duplicate result</a>
            <a href="mailto:test@example.com">Mail link</a>
            <a href="#fragment">Fragment link</a>
          </body>
        </html>
    "##;

    let results = parse_brave_results(html, 5);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "First result");
    assert_eq!(results[0].1, "https://example.com/page1");
}

#[test]
fn extract_page_text_builds_markdown_like_article() {
    let html = r#"
        <html>
          <body>
            <h1>Main Title</h1>
            <h2>Subtitle text</h2>
            <p>This is the first paragraph with more than thirty characters.</p>
            <p>This is the second paragraph, also long enough to be kept.</p>
          </body>
        </html>
    "#;

    let markdown = extract_page_text(html);

    assert!(markdown.starts_with("# Main Title"), "Markdown should start with H1 title");
    assert!(
        markdown.contains("## Subtitle text"),
        "Markdown should contain the subtitle as H2"
    );
    assert!(
        markdown.contains("This is the first paragraph"),
        "Markdown should include the first paragraph text"
    );
    assert!(
        markdown.contains("This is the second paragraph"),
        "Markdown should include the second paragraph text"
    );
}

/// Manual, non-mocked test: runs the `web.search` intent and prints the result.
/// Run with: `CC=clang cargo test manual_web_search_prints_markdown -- --ignored`
#[test]
#[ignore]
fn manual_web_search_prints_markdown() {
    let input = IntentInput {
        data: Some(String::from("Bob Dylan Blood on the Tracks")),
        args: vec![],
    };

    let result = Search.execute(input);

    println!("\n--- manual_web_search_prints_markdown ---");
    match result.status {
        ExecutionStatus::Ok => {
            let markdown_value = result
                .result
                .as_ref()
                .and_then(|v| v.get("data"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            println!("Status: Ok");
            println!("Markdown output (first 1000 chars):\n{}\n",
                &markdown_value.chars().collect::<String>());
        }
        ExecutionStatus::Error => {
            println!("Status: Error");
            println!("Error: {:?}", result.error);
        }
    }
}


