use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use crate::{Config, Result};

pub fn hash(raw: &str) -> String {
    format!("{:032x}", md5::compute(raw))
}

pub fn send_html_email(config: &Config, to: &str, subject: &str, body: String) -> Result<()> {
    let message = Message::builder()
        .from(config.smtp_from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;

    let mailer = SmtpTransport::relay(&config.smtp_host)?
        .credentials(Credentials::new(
            config.smtp_user.clone(),
            config.smtp_password.clone(),
        ))
        .port(config.smtp_port)
        .build();

    mailer.send(&message)?;

    Ok(())
}
