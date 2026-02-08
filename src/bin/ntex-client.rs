use futures::stream::{self, StreamExt};
use ntex::client::Client;
use std::time::Instant;

#[ntex::main]
async fn main() {
    let client = Client::new().await;
    let url = "http://127.0.0.1:8080";

    let total_requests = 500_000;
    let concurrency = 12; // Adjust this based on your P-cores

    println!(
        "Starting benchmark: {} requests, concurrency {}...",
        total_requests, concurrency
    );
    let start = Instant::now();

    stream::iter(0..total_requests)
        .map(|_| {
            let client = client.clone();
            async move {
                // We don't even read the body to maximize raw RPS
                let _ = client.get(url).send().await;
            }
        })
        .buffer_unordered(concurrency) // This is the magic for throughput
        .collect::<Vec<_>>()
        .await;

    let duration = start.elapsed();
    let rps = total_requests as f64 / duration.as_secs_f64();

    println!("-----------------------------------");
    println!("Finished in: {:?}", duration);
    println!("Requests/sec: {:.2}", rps);
}
