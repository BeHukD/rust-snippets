// snippet_net_http_20260213T1606.rs
// Topic: HTTP client with reqwest (async)
// Demonstrates making GET/POST requests, handling JSON responses.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // Example 1: GET JSON from a public API
    let repo_url = "https://api.github.com/repos/BeHukD/rust-snippets";
    let resp: serde_json::Value = client.get(repo_url)
        .header("User-Agent", "rust_snippets_example")
        .send()
        .await?
        .json()
        .await?;

    println!("Repo: {} (stars: {})", resp["full_name"], resp["stargazers_count"]);

    // Example 2: POST JSON payload
    #[derive(Serialize)]
    struct NewIssue {
        title: String,
        body: Option<String>,
    }

    let new_issue = NewIssue {
        title: "Test issue from Rust snippet".to_string(),
        body: Some("This issue was created via reqwest in a snippet.".to_string()),
    };

    // Uncomment the lines below to actually create an issue (requires auth):
    // let token = "YOUR_GITHUB_TOKEN";
    // let create_resp = client.post(&format!("{}/issues", repo_url))
    //     .bearer_auth(token)
    //     .json(&new_issue)
    //     .send()
    //     .await?;
    // println!("Created issue: {}", create_resp.status());

    // Example 3: Simple GET plain text
    let plain_resp = client.get("https://example.com")
        .send()
        .await?
        .text()
        .await?;
    println!("Example.com first 200 chars:\n{}", &plain_resp[..plain_resp.len().min(200)]);

    Ok(())
}

/*
Dependencies for Cargo.toml:

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

Notes:
- For HTTPS/rustls, reqwest uses native-tls or rustls; the default features are fine.
- To run this snippet, you need an async runtime (tokio) and the features enabled.
- The GitHub API example uses a public endpoint; to write (POST), you need an auth token.

Run:
cargo run --bin snippet_net_http_20260213T1606.rs
*/
