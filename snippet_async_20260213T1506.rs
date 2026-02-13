// snippet_async_20260213T1506.rs
// Topic: Asynchronous programming with Tokio (async/await)
// This snippet demonstrates a simple async function using Tokio's runtime.

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting async example...");
    let result = do_async_work().await;
    println!("Result: {}", result);
}

/// Simulates an asynchronous operation (e.g., I/O or network request).
/// Returns a static string after a 1-second delay.
async fn do_async_work() -> &'static str {
    // In real-world code, this could be a database query, HTTP request, etc.
    sleep(Duration::from_secs(1)).await;
    "Async task completed"
}

/*
To build and run this snippet standalone:

1. Add to Cargo.toml dependencies:
   [dependencies]
   tokio = { version = "1", features = ["full"] }

2. Run:
   cargo run --bin snippet_async_20260213T1506.rs

Or simply check with:
   cargo check
*/
