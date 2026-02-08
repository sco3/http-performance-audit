use ntex::client::Client;
use std::time::Instant;

#[ntex::main]
async fn main() {
    let client = Client::new().await;
    let url = "http://127.0.0.1:8080";
    
    let workers = 12; 
    let reqs_per_worker = 50_000;
    let total = workers * reqs_per_worker;

    println!("🚀 Ntex + Neon-Uring Mode (No Tokio)");
    let start = Instant::now();

    let mut tasks = Vec::new();
    for _ in 0..workers {
        let client = client.clone();
        tasks.push(ntex::rt::spawn(async move {
            for _ in 0..reqs_per_worker {
                if let Ok(res) = client.get(url).send().await {
                    let _ = res.body().await;
                }
            }
        }));
    }

    for t in tasks { let _ = t.await; }

    let dur = start.elapsed();
    println!("-----------------------------------");
    println!("Final Rust RPS: {:.2}", total as f64 / dur.as_secs_f64());
}
