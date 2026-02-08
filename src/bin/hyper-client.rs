use std::time::Instant;
use std::sync::Arc;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{BodyExt, Empty};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    // The legacy client in hyper-util is the fastest way to get a pool in Hyper 1.0
    let client = Arc::new(
        Client::builder(TokioExecutor::new())
            .build_http::<Empty<Bytes>>()
    );
    
    let url: hyper::Uri = "http://127.0.0.1:8080".parse().unwrap();
    let workers = 12;
    let reqs_per_worker = 40_000;
    let total = workers * reqs_per_worker;

    println!("⚡ Hyper Raw Mode: 12 workers, {} total requests", total);
    let start = Instant::now();

    let mut tasks = Vec::new();
    for _ in 0..workers {
        let client = client.clone();
        let url = url.clone();
        tasks.push(tokio::spawn(async move {
            for _ in 0..reqs_per_worker {
                if let Ok(res) = client.get(url.clone()).await {
                    // We must consume the body frames to return connection to pool
                    let mut body = res.into_body();
                    while let Some(frame) = body.frame().await {
                        let _ = frame;
                    }
                }
            }
        }));
    }

    for t in tasks { let _ = t.await; }

    let dur = start.elapsed();
    println!("-----------------------------------");
    println!("Hyper RPS: {:.2}", total as f64 / dur.as_secs_f64());
}
