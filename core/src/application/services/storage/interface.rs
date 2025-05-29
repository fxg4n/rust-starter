use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn upload_file(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<String>;
    async fn download_file(&self, bucket: &str, key: &str) -> Result<Vec<u8>>;
    async fn delete_file(&self, bucket: &str, key: &str) -> Result<()>;
    async fn get_file_url(&self, bucket: &str, key: &str) -> Result<String>;
}