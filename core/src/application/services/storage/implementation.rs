use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::drivers::services::aws::s3::S3Client;
use super::interface::StorageService;

pub struct DefaultStorageService {
    s3_client: Arc<S3Client>,
}

impl DefaultStorageService {
    pub fn new(s3_client: S3Client) -> Self {
        Self {
            s3_client: Arc::new(s3_client),
        }
    }
}

#[async_trait]
impl StorageService for DefaultStorageService {
    async fn upload_file(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<String> {
        self.s3_client.upload_object(bucket, key, data).await
    }

    async fn download_file(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        self.s3_client.download_object(bucket, key).await
    }

    async fn delete_file(&self, bucket: &str, key: &str) -> Result<()> {
        self.s3_client.delete_object(bucket, key).await
    }

    async fn get_file_url(&self, bucket: &str, key: &str) -> Result<String> {
        self.s3_client.get_object_url(bucket, key).await
    }
}