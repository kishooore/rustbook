use anyhow::{Context, Result};
use aws_sdk_s3::Client;
use tracing::info;

pub struct S3Downloader {
    client: Client
}

impl S3Downloader {
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        let client = aws_sdk_s3::Client::new(&config);
        Ok(Self {client})
    }

    pub async fn download_file(&self, bucket: &str, key: &str, destination: &str) -> Result<()> {
        info!("Preparing to download the file from S3.");

        let response = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .context("Failed to get response from S3.")?;

        let mut file = tokio::fs::File::create(destination)
            .await
            .context("Failed to create destination file.")?;

        let mut stream = response.body.into_async_read();

        tokio::io::copy(&mut stream, &mut file)
            .await?;

        info!(destination = destination, key = key, "File successfully downloaded.");

        Ok(())
    }
}