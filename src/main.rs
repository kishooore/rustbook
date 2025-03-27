mod downloader;

use anyhow::Result;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tokio::join!(task_one(), task_two());

    Ok(())
}

async fn task_one() {
    println!("starting task one.");
    sleep(Duration::from_secs(3)).await;
    println!("completed task one.");
}

async fn task_two() {
    println!("starting task two.");
    sleep(Duration::from_secs(1)).await;
    println!("completed task two.");
}