use anyhow::Result;
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
    transport::smtp::{
        authentication::Credentials,
    },
};
use crate::config::services::SmtpConfig;
use super::interface::EmailTransporter;

pub struct SmtpMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

#[async_trait]
impl EmailTransporter for SmtpMailer {
    async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        html_content: &str,
    ) -> Result<()> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(format!("{} <{}>", to_name, to_email).parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(html_content.to_string())?;

        self.transport.send(email).await?;
        Ok(())
    }
}

impl SmtpMailer {
    pub async fn new(config: &SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(
            config.username.clone(),
            config.password.clone(),
        );

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?
            .port(config.port)
            .credentials(creds)
            .build();

        Ok(Self {
            transport,
            from_email: config.from_email.clone(),
            from_name: config.from_name.clone(),
        })
    }
}