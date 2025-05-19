use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(
        &self, 
        to_email: &str, 
        to_name: &str, 
        subject: &str, 
        template_path: &str, 
        params: &[(&str, &str)]
    ) -> Result<()>;
}