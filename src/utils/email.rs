use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::client::TlsParameters;
use std::sync::Arc;
use std::env;

#[derive(Clone)]
pub struct EmailService {
    smtp_transport: Arc<SmtpTransport>,
    from_email: String,
}

impl EmailService {
    pub fn new() -> Self {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid port number");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let from_email = env::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL must be set");

        let creds = Credentials::new(smtp_username, smtp_password);

        let tls_parameters = TlsParameters::new(smtp_host.clone())
            .unwrap();

        let smtp_transport = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .port(smtp_port)
            .tls(lettre::transport::smtp::client::Tls::Wrapper(tls_parameters))
            .credentials(creds)
            .build();

        Self {
            smtp_transport: Arc::new(smtp_transport),
            from_email,
        }
    }

    pub fn send_verification_code(&self, to_email: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject("Your Verification Code")
            .body(format!("Your verification code is: {}", code))?;

        self.smtp_transport.send(&email)?;
        Ok(())
    }
}
