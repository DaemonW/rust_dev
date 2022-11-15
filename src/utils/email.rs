use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub fn send_mail(
    receiver: &str,
    title: &str,
    body: &str,
    poster: &str,
    credential: Credentials,
    smtp_server: &str,
) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from(poster.parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(title)
        .body(String::from(body))?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(credential)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn check_smtp(username: &str, password: &str, smtp_server: &str) -> Result<(), Box<dyn Error>> {
    let c = Credentials::new(username.into(), password.into());
    // Open a remote connection to gmail
    check_auth(c, smtp_server)
}

fn check_auth(credential: Credentials, smtp_server: &str) -> Result<(), Box<dyn Error>> {
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(credential)
        .build();
    match mailer.test_connection() {
        Ok(_) => Ok(()),
        Err(e) => {
            let msg = format!("{}", e);
            Err(msg.into())
        }
    }
}
