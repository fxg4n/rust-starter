use anyhow::Result;
use async_trait::async_trait;
use std::fs;
use crate::drivers::services::smtp_mailer::EmailTransporter;
use super::interface::MailService;

pub struct DefaultMailService {
    email_transporter: Box<dyn EmailTransporter>,
}

impl DefaultMailService {
    pub fn new(email_transporter: Box<dyn EmailTransporter>) -> Self {
        Self { email_transporter }
    }
    
    fn load_and_format_template(&self, template_path: &str, params: &[(&str, &str)]) -> Result<String> {
        let mut content = fs::read_to_string(template_path)?;
        for (key, value) in params {
            content = content.replace(&format!("{{{{{}}}}}", key), value);
        }
        Ok(content)
    }
}

#[async_trait]
impl MailService for DefaultMailService {
    async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        template_path: &str,
        params: &[(&str, &str)]
    ) -> Result<()> {
        let content = self.load_and_format_template(template_path, params)?;
        self.email_transporter.send_email(
            to_email,
            to_name,
            subject,
            &content,
        ).await
    }
}