// SPDX-License-Identifier: MPL-2.0
//
// Övervakt
// Copyright © 2022 Brendan Molloy <brendan@bbqsrc.net>
//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this file,
//   You can obtain one at https://mozilla.org/MPL/2.0/.
//
// ---
//
// Fork of: Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::{config::notify, APP_CONF};

static TEXT_MESSAGE_TRUNCATED_INDICATOR: &str = "[..]";

const TEXT_MESSAGE_MAXIMUM_LENGTH: usize = 1000;

static TWILIO_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct TwilioNotifier;

#[derive(Debug, thiserror::Error)]
#[error("Failed to deliver message to phone numbers: {0:?}")]
pub struct Error(Vec<(String, reqwest::Error)>);

impl Notifier for TwilioNotifier {
    type Config = notify::Twilio;
    type Error = Error;

    fn attempt(twilio: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        // Build up the message text
        let mut message = String::new();

        if notification.startup {
            message.push_str("Startup alert for: ");
        } else if !notification.changed {
            message.push_str("Reminder for: ");
        }

        message.push_str(&format!("{}\n\n", APP_CONF.branding.page_title));
        message.push_str(&format!("Status: {:?}\n", notification.status));
        message.push_str(&format!("Nodes: {}\n", &notification.replicas.join(", ")));
        message.push_str(&format!("Time: {}\n", &notification.time));

        // Trim down message to a maximum length? (most SMS receivers and networks support \
        //   up to 1600 characters by re-building message segments)
        if message.len() > TEXT_MESSAGE_MAXIMUM_LENGTH {
            tracing::debug!(
                "message for Twilio notification is too long, trimming to length: {}",
                TEXT_MESSAGE_MAXIMUM_LENGTH
            );

            message.truncate(TEXT_MESSAGE_MAXIMUM_LENGTH - TEXT_MESSAGE_TRUNCATED_INDICATOR.len());

            message.push_str(TEXT_MESSAGE_TRUNCATED_INDICATOR);
        }

        tracing::debug!("will send Twilio notification with message: {}", &message);

        let mut failures = vec![];

        for to_number in &twilio.to {
            // Build form parameters
            let mut params = HashMap::new();

            params.insert("MessagingServiceSid", &twilio.service_sid);
            params.insert("To", to_number);
            params.insert("Body", &message);

            // Submit message to Twilio
            let response = TWILIO_HTTP_CLIENT
                .post(&generate_api_url(&twilio.account_sid))
                .basic_auth(
                    twilio.account_sid.as_str(),
                    Some(twilio.auth_token.as_str()),
                )
                .form(&params)
                .send();

            match response {
                Ok(response) => match response.error_for_status() {
                    Ok(_) => {}
                    Err(err) => {
                        failures.push((to_number.to_string(), err));
                    }
                },
                Err(err) => {
                    failures.push((to_number.to_string(), err));
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(Error(failures))
        }
    }

    fn can_notify(config: &Self::Config, notification: &Notification<'_>) -> bool {
        notification.expected(config.reminders_only)
    }

    fn name() -> &'static str {
        "twilio"
    }
}

fn generate_api_url(account_sid: &str) -> String {
    format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        account_sid
    )
}
