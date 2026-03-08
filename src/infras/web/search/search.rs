use std::collections::HashSet;
use std::error::Error;

use percent_encoding::percent_decode_str;
use scraper::{Html, Selector};
use serde_json::json;
use tokio::runtime::Runtime;

use crate::intent::Intent;
use crate::types::{ExecutionStatus, IntentInput, IntentResult};

const RESULTS_LIMIT: usize = 5;

pub struct Search;

impl Intent for Search {
    fn name(&self) -> &'static str {
        "search"
    }

    fn path(&self) -> &'static str {
        "web.search"
    }

    fn description(&self) -> &'static str {
        "Search the web using Brave and return a Markdown summary buffer."
    }

    fn execute(&self, input: IntentInput) -> IntentResult {
        let query = match extract_query(input) {
            Ok(q) => q,
            Err(msg) => {
                return IntentResult {
                    status: ExecutionStatus::Error,
                    result: None,
                    error: Some(msg),
                }
            }
        };

        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return IntentResult {
                    status: ExecutionStatus::Error,
                    result: None,
                    error: Some(format!("Failed to create async runtime: {}", e)),
                }
            }
        };

        match rt.block_on(run_search(&query)) {
            Ok(markdown) => IntentResult {
                status: ExecutionStatus::Ok,
                result: Some(json!({ "markdown": markdown })),
                error: None,
            },
            Err(e) => IntentResult {
                status: ExecutionStatus::Error,
                result: None,
                error: Some(format!("Failed to execute web.search: {}", e)),
            },
        }
    }
}

fn extract_query(input: IntentInput) -> Result<String, String> {
    if let Some(data) = input.data {
        let trimmed = data.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Some(first) = input.args.first() {
        let trimmed = first.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    Err("Missing query for web.search intent".to_string())
}

pub fn parse_brave_results(body: &str) -> Vec<(String, String)> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("a").unwrap();
    let mut seen = HashSet::new();
    let mut results = Vec::new();
    let blacklist = [
        "youtube.com",
        "youtu.be",
        "twitter.com",
        "facebook.com",
        "instagram.com",
        "linkedin.com",
        "amazon.com",
        "spotify.com",
        "apple.com",
        "wikipedia.org",
    ];

    for element in document.select(&selector) {
        if results.len() >= RESULTS_LIMIT {
            break;
        }

        if let Some(href) = element.value().attr("href") {
            let href = href.trim();
            if href.is_empty() {
                continue;
            }
            if href.starts_with('#')
                || href.starts_with("javascript:")
                || href.starts_with("mailto:")
            {
                continue;
            }
            if !(href.starts_with("http://") || href.starts_with("https://")) {
                continue;
            }
            if href.contains("search.brave.com")
                || href.contains("cdn.search.brave.com")
                || href.contains("imgs.search.brave.com")
                || href.contains("tiles.search.brave.com")
            {
                continue;
            }

            let title = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .replace('\n', " ")
                .trim()
                .to_string();
            if title.is_empty() {
                continue;
            }

            let decoded = percent_decode_str(href).decode_utf8_lossy().to_string();
            // skip known non-article hosts
            if let Ok(parsed) = reqwest::Url::parse(&decoded) {
                if let Some(host) = parsed.host_str() {
                    let mut skip_host = false;
                    for b in &blacklist {
                        if host.ends_with(b) {
                            skip_host = true;
                            break;
                        }
                    }
                    if skip_host {
                        continue;
                    }
                }
            }
            if seen.contains(&decoded) {
                continue;
            }
            seen.insert(decoded.clone());
            results.push((title, decoded));
        }
    }

    results
}

pub fn extract_page_text(body: &str) -> String {
    let document = Html::parse_document(body);
    // Try a set of selectors in order to capture main article text while avoiding nav/footers
    let title = document
        .select(&Selector::parse("h1#firstHeading,h1,header h1").unwrap())
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
        .unwrap_or_default();

    let mut subtitle = document
        .select(&Selector::parse("h2,header h2,.subtitle,.lead").unwrap())
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
        .unwrap_or_default();
    if subtitle.eq_ignore_ascii_case("contents") {
        subtitle.clear();
    }

    // paragraph selectors: prefer article p, mw-content-text p, main p, then any p
    let mut paragraphs = Vec::new();
    let para_selectors = [
        "article p",
        "#mw-content-text p",
        "main p",
        "div[class*='content'] p",
        "p",
    ];

    for sel in &para_selectors {
        let s = Selector::parse(sel).unwrap();
        for p in document.select(&s) {
            let txt = p.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if !txt.is_empty() {
                // avoid very short boilerplate lines
                if txt.len() > 30 || paragraphs.is_empty() {
                    paragraphs.push(txt);
                }
            }
        }
        if !paragraphs.is_empty() {
            break;
        }
    }

    // build markdown-formatted output: title becomes an H1, subtitle an H2,
    // paragraphs are left as-is separated by blank lines
    let mut content = String::new();
    if !title.is_empty() {
        content.push_str(&format!("# {}\n\n", title));
    }
    if !subtitle.is_empty() {
        content.push_str(&format!("## {}\n\n", subtitle));
    }
    for p in paragraphs {
        content.push_str(&p);
        content.push_str("\n\n");
    }
    content.trim().to_string()
}

async fn fetch_all_contents(
    client: &reqwest::Client,
    items: &[(String, String)],
) -> Result<Vec<(String, String, String)>, Box<dyn Error + Send + Sync>> {
    use futures::future::join_all;

    let futures = items.iter().map(|(title, url)| {
        let client = client.clone();
        let title = title.clone();
        let url = url.clone();
        async move {
            let body = client
                .get(&url)
                .header(reqwest::header::USER_AGENT, "libintent-web.search/0.1")
                .send()
                .await?
                .text()
                .await?;
            let content = extract_page_text(&body);
            Ok((title, url, content))
                as Result<(String, String, String), Box<dyn Error + Send + Sync>>
        }
    });

    let all = join_all(futures).await;
    let mut results = Vec::new();
    for item in all {
        results.push(item?);
    }
    Ok(results)
}

async fn run_search(query: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://search.brave.com/search?q={}",
        query.replace(' ', "+")
    );

    let resp = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "libintent-web.search/0.1")
        .send()
        .await?;

    let body = resp.text().await?;
    let results = parse_brave_results(&body);

    let full_results = fetch_all_contents(&client, &results).await?;

    let mut markdown = String::new();
    for (i, (title, url, content)) in full_results.iter().enumerate() {
        markdown.push_str(&format!("{}. [{}]({})\n\n", i + 1, title, url));
        if !content.is_empty() {
            markdown.push_str(content);
            markdown.push_str("\n\n");
        }
        markdown.push_str("---\n\n");
    }

    Ok(markdown.trim().to_string())
}

