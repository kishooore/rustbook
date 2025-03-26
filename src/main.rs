mod downloader;

use anyhow::Result;
use tracing::info;
use downloader::S3Downloader;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Cloud file downloader starting...");

    let downloader = S3Downloader::new().await?;

    downloader.download_file(
            "kishore-cloudera-dbus-dev",
            "CmlWorkloads/037c789f-4033-4633-8038-b1868bb114f6",
            "/Users/kishore/037c789f-4033-4633-8038-b1868bb114f6")
        .await?;
    Ok(())
}