mod downloader;

use anyhow::Result;
use tracing::info;
use downloader::S3Downloader;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Cloud file downloader starting...");

    let downloader = S3Downloader::new().await?;
    Ok(())
}