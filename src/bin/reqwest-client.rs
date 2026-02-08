use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Reqwest Client already uses Arc internally for its pool,
    // but we wrap it to be explicit for the tasks.
    let client = Arc::new(reqwest::Client::new());
    let url = "http://127.0.0.1:8080";

    let workers_count = 12; // Your i5-12500H cores
    let reqs_per_worker = 20_000; // Lower total so we don't wait forever
    let total_requests = workers_count * reqs_per_worker;

    println!(
        "🐢 Testing 'Pathetic' Reqwest: {} requests...",
        total_requests
    );
    let start = Instant::now();

    let mut tasks = Vec::new();

    for _ in 0..workers_count {
        let client = client.clone();
        tasks.push(tokio::spawn(async move {
            for _ in 0..reqs_per_worker {
                // Reqwest requires you to await the send, then await the body
                if let Ok(res) = client.get(url).send().await {
                    let _ = res.bytes().await; // Drain the body
                }
            }
        }));
    }

    for task in tasks {
        let _ = task.await;
    }

    let duration = start.elapsed();
    println!("-----------------------------------");
    println!(
        "Reqwest RPS: {:.2}",
        total_requests as f64 / duration.as_secs_f64()
    );
}
