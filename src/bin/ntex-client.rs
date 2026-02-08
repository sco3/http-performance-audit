use futures::stream::{self, StreamExt};
use ntex::client::Client;
use std::time::Instant;

#[ntex::main]
async fn main() {
    // In v3.x, initialization is async.
    // This creates the internal connector and reactor bindings.
    let client = Client::new().await;
    let url = "http://127.0.0.1:8080";

    let total_requests = 500_000;
    let concurrency = 128; // Increased to fill the 500 connections in the pool

    println!(
        "🚀 Ntex 3.1 Client: 500k reqs @ {} concurrency",
        concurrency
    );
    let start = Instant::now();

    stream::iter(0..total_requests)
        .map(|_| {
            let client = client.clone();
            async move {
                // To keep the connection alive, we send the request
                // and await the response object.
                if let Ok(mut response) = client.get(url).send().await {
                    // IMPORTANT: You must wait for the body to finish
                    // or the connection will be dropped/closed.
                    let _ = response.body().await;
                }
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;

    let duration = start.elapsed();
    println!("-----------------------------------");
    println!(
        "Requests/sec: {:.2}",
        total_requests as f64 / duration.as_secs_f64()
    );
}
