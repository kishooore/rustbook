use anyhow::Result;
use aws_sdk_s3::Client;
use tracing::info;

pub struct S3Downloader {
    client: Client
}

impl S3Downloader {
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);
        Ok(Self {client})
    }

    pub async fn download_file(&self, bucket: &str, ket: &str, desination: &str) -> Result<()> {
        info!("Preparing to download the file from S3.");
        Ok(())
    }
}