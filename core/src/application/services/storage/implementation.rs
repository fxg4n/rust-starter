use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::drivers::services::aws::s3::S3Client;
use crate::config::services::AwsConfig;
use super::interface::StorageService;

pub struct DefaultStorageService {
    s3_client: Arc<S3Client>,
    config: Arc<AwsConfig>,
}

impl DefaultStorageService {
    pub async fn new(config: AwsConfig) -> Result<Self> {
        let s3_client = S3Client::new(&config).await?;
        
        Ok(Self {
            s3_client: Arc::new(s3_client),
            config: Arc::new(config),
        })
    }
}

#[async_trait]
impl StorageService for DefaultStorageService {
    async fn upload_file(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<String> {
        let bucket = bucket.to_string();
        let bucket = if bucket.is_empty() { &self.config.s3_bucket } else { &bucket };
        self.s3_client.upload_object(bucket, key, data).await
    }

    async fn download_file(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        let bucket = bucket.to_string();
        let bucket = if bucket.is_empty() { &self.config.s3_bucket } else { &bucket };
        self.s3_client.download_object(bucket, key).await
    }

    async fn delete_file(&self, bucket: &str, key: &str) -> Result<()> {
        let bucket = bucket.to_string();
        let bucket = if bucket.is_empty() { &self.config.s3_bucket } else { &bucket };
        self.s3_client.delete_object(bucket, key).await
    }

    async fn get_file_url(&self, bucket: &str, key: &str) -> Result<String> {
        let bucket = bucket.to_string();
        let bucket = if bucket.is_empty() { &self.config.s3_bucket } else { &bucket };
        self.s3_client.get_object_url(bucket, key).await
    }
}