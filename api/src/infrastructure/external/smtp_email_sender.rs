use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

use crate::{application::traits::email_sender::EmailSender, domain::authentication::email::Email};

pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub tls: String,
}

pub struct SmtpEmailSender {
    transport: AsyncSmtpTransport<lettre::Tokio1Executor>,
    from: Mailbox,
}

impl SmtpEmailSender {
    pub fn try_new(config: SmtpConfig) -> Result<Self, anyhow::Error> {
        let creds = Credentials::new(config.username.to_string(), config.password.to_string());

        let tls = match config.tls.as_str() {
            "required" => Tls::Required(TlsParameters::new(config.host.clone())?),
            "none" => Tls::None,
            _ => return Err(anyhow::anyhow!("invalid SMTP_TLS: {}", config.tls)),
        };

        let mailbox = config.from.parse()?;
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?
            .credentials(creds)
            .port(config.port)
            .tls(tls)
            .build();

        Ok(Self {
            transport: mailer,
            from: mailbox,
        })
    }
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send(&self, to: &Email, subject: &str, body: String) -> Result<(), anyhow::Error> {
        let mailer = Message::builder()
            .to(to.as_ref().parse()?)
            .from(self.from.clone())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        self.transport.send(mailer).await?;
        Ok(())
    }
}
