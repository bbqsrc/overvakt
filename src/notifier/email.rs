// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use lettre::transport::smtp;
use lettre::transport::smtp::{
    authentication::Credentials, SmtpTransport
};
use lettre::Transport;
use lettre::message::{MessageBuilder, Mailbox};

use super::generic::{GenericNotifier, Notification, DISPATCH_TIMEOUT_SECONDS};
use crate::config::config::ConfigNotifyEmail;
use crate::APP_CONF;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There was an error parsing an address")]
    Address(#[from] lettre::address::AddressError),

    #[error("There was an error handling creation of the email")]
    Email(#[from] lettre::error::Error),

    #[error("There was an SMTP error")]
    Smtp(#[from] smtp::Error),
}

pub struct EmailNotifier;

impl GenericNotifier for EmailNotifier {
    type Config = ConfigNotifyEmail;
    type Error = Error;

    fn attempt(email_config: &ConfigNotifyEmail, notification: &Notification) -> Result<(), Self::Error> {
        let nodes_label = notification.replicas.join(", ");

        // Build up the message text
        let mut message = String::new();

        if notification.startup == true {
            message.push_str(&format!(
                "Status startup alert from: {}\n",
                APP_CONF.branding.page_title
            ));
        } else if notification.changed == true {
            message.push_str(&format!(
                "Status change report from: {}\n",
                APP_CONF.branding.page_title
            ));
        } else {
            message.push_str(&format!(
                "Status unchanged reminder from: {}\n",
                APP_CONF.branding.page_title
            ));
        }

        message.push_str("\n--\n");
        message.push_str(&format!("Status: {:?}\n", notification.status));
        message.push_str(&format!("Nodes: {}\n", &nodes_label));
        message.push_str(&format!("Time: {}\n", &notification.time));
        message.push_str(&format!("URL: {}", APP_CONF.branding.page_url.as_str()));

        message.push_str("\n--\n");
        message.push_str("\n");
        message.push_str("To unsubscribe, please edit your status page configuration.");

        debug!("will send email notification with message: {}", &message);

        // Build up the email
        let email_message = MessageBuilder::new()
            .to(email_config.to.as_str().parse()?)
            .from(Mailbox::new(
                Some(APP_CONF.branding.page_title.to_string()),
                email_config.from.as_str().parse()?
            ))
            .subject(format!(
                "{} | {}",
                notification.status.as_str().to_uppercase(),
                &nodes_label
            ))
            .body(message)?;

        // Deliver the message
        let transport = acquire_transport(
            &email_config.smtp_host,
            email_config.smtp_port,
            email_config.smtp_username.to_owned(),
            email_config.smtp_password.to_owned(),
            email_config.smtp_encrypt,
        )?;

        transport.send(&email_message)?;
        
        Ok(())
    }

    fn can_notify(email_config: &ConfigNotifyEmail, notification: &Notification) -> bool {
        notification.expected(email_config.reminders_only)
    }

    fn name() -> &'static str {
        "email"
    }
}

fn acquire_transport(
    smtp_host: &str,
    smtp_port: u16,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    smtp_encrypt: bool,
) -> Result<SmtpTransport, smtp::Error> {
    let relay = if smtp_encrypt {
        SmtpTransport::starttls_relay(&format!("{}:{}", smtp_host, smtp_port))?
    } else {
        SmtpTransport::relay(&format!("{}:{}", smtp_host, smtp_port))?
    };

    let relay = relay.timeout(Some(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS)));
    let relay = match (smtp_username, smtp_password) {
        (Some(username), Some(password)) => {
            relay.credentials(Credentials::new(username, password))
        }
        _ => relay
    };

    Ok(relay.build())
}
