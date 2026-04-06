use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};

use crate::{application::traits::email_sender::EmailSender, domain::authentication::email::Email};

pub struct SmtpEmailSender {
    transport: AsyncSmtpTransport<lettre::Tokio1Executor>,
    from: Mailbox,
}

impl SmtpEmailSender {
    pub fn try_new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        from: &str,
    ) -> Result<Self, anyhow::Error> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let mailbox = from.parse()?;
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(host)?
            .credentials(creds)
            .port(port)
            .build();

        Ok(Self {
            transport: mailer,
            from: mailbox,
        })
    }
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), anyhow::Error> {
        let mailer = Message::builder()
            .to(to.as_ref().parse()?)
            .from(self.from.clone())
            .subject(subject)
            .body(body.to_string())?;
        self.transport.send(mailer).await?;
        Ok(())
    }
}
