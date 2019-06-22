use lettre_email::EmailBuilder;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use std::env;

pub struct Mailer{
    transport: SmtpTransport
}

impl Mailer {
    pub fn new() -> Self {
        let transport = SmtpClient::new_simple(env::var("SMTP_HOST").unwrap().as_str())
            .expect("Unable to create SmtpClient.")
            .credentials(
                Credentials::new(
                    env::var("SMTP_USERNAME").unwrap(),
                    env::var("SMTP_PASSWORD").unwrap()
                )
            )
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
            .transport();

        Mailer{transport}
    }

    pub fn send(&mut self, content: String) {
        let email = EmailBuilder::new()
            .from("noreply@wizzair-flight-finder.rs".to_string())
            .to("kunicmarko20@gmail.com".to_string())
            .subject("Wizzair Flight Finder")
            .html(content)
            .build()
            .expect("Unable to build email.");

        self.transport.send(email.into()).expect("Unable to send email.");
    }
}