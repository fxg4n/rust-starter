use anyhow::Result;
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
    message::{header, MultiPart, SinglePart},
    transport::smtp::{
        authentication::Credentials,
    },
};

use crate::config::services::SmtpConfig;

pub struct SmtpClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

impl SmtpClient {
    pub async fn new() -> Result<Self> {
        let config = SmtpConfig::from_env()?;
        
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

    pub async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        text_content: &str,
        html_content: Option<&str>,
    ) -> Result<()> {
        let to_address = format!("{} <{}>", to_name, to_email).parse()?;
        let from_address = format!("{} <{}>", self.from_name, self.from_email).parse()?;

        let message_builder = Message::builder()
            .from(from_address)
            .to(to_address)
            .subject(subject);

        let message = if let Some(html) = html_content {
            message_builder.multipart(
                MultiPart::alternative()
                    .singlepart(SinglePart::plain(text_content.to_string()))
                    .singlepart(SinglePart::html(html.to_string()))
            )?
        } else {
            message_builder.body(text_content.to_string())?
        };

        self.transport.send(message).await?;

        Ok(())
    }

    pub async fn send_email_with_attachment(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        text_content: &str,
        html_content: Option<&str>,
        attachment: Vec<u8>,
        filename: &str,
        content_type: &str,
    ) -> Result<()> {
        let to_address = format!("{} <{}>", to_name, to_email).parse()?;
        let from_address = format!("{} <{}>", self.from_name, self.from_email).parse()?;

        let message_builder = Message::builder()
            .from(from_address)
            .to(to_address)
            .subject(subject);

        let attachment_part = SinglePart::builder()
            .header(header::ContentType::parse(content_type)?)
            .header(header::ContentDisposition::attachment(filename))
            .body(attachment);

        let message = if let Some(html) = html_content {
            message_builder.multipart(
                MultiPart::mixed()
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(SinglePart::plain(text_content.to_string()))
                            .singlepart(SinglePart::html(html.to_string()))
                    )
                    .singlepart(attachment_part)
            )?
        } else {
            message_builder.multipart(
                MultiPart::mixed()
                    .singlepart(SinglePart::plain(text_content.to_string()))
                    .singlepart(attachment_part)
            )?
        };

        self.transport.send(message).await?;

        Ok(())
    }
}