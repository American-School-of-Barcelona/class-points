use lettre::message::{Mailbox, Message, header};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use std::env;

pub async fn send(to: &str, subject: &str, body: String) -> Result<(), crate::Error> {
    let user = env::var("GMAIL_USER")?;
    let pass = env::var("GMAIL_PASS")?;
    let creds = Credentials::new(user.clone(), pass);

    let email = Message::builder()
        .from(Mailbox::new(
            Some("ASBCS Authenticator".into()),
            user.parse()?,
        ))
        .to(Mailbox::new(None, to.parse()?))
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(body)?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    Ok(())
}
