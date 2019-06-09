use lettre_email::EmailBuilder;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;

pub struct Mailer{
    transport: SmtpTransport
}

impl Mailer {
    pub fn new() -> Self {
        let transport = SmtpClient::new_simple(env!("SMTP_HOST"))
            .expect("Unable to create SmtpClient.")
            .credentials(
                Credentials::new(
                    env!("SMTP_USERNAME").to_string(),
                    env!("SMTP_PASSWORD").to_string()
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