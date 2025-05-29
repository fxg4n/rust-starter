use anyhow::Result;
use aws_sdk_s3::{Client, Config};
use aws_config::SdkConfig;
use crate::config::services::AwsConfig;

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new(config: &AwsConfig) -> Result<Self> {
        let aws_config = aws_config::from_env()
            .credentials_provider(aws_config::Credentials::new(
                &config.access_key_id,
                &config.secret_access_key,
                None,
                None,
                "s3-client",
            ))
            .region(aws_config::Region::new(config.region.clone()))
            .load()
            .await;

        let s3_client = Client::new(&aws_config);
        
        Ok(Self {
            client: s3_client,
        })
    }

    pub async fn upload_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<String> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data.into())
            .send()
            .await?;

        Ok(format!("s3://{}/{}", bucket, key))
    }

    pub async fn download_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(response.body.collect().await?.to_vec())
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_object_url(&self, bucket: &str, key: &str) -> Result<String> {
        let presigned_url = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
                std::time::Duration::from_secs(3600),
            )?)
            .await?;

        Ok(presigned_url.uri().to_string())
    }
}