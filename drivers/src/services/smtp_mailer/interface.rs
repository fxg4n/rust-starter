use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EmailTransporter: Send + Sync {
    async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        html_content: &str,
    ) -> Result<()>;
}