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
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::config::notify;
use crate::prober::status::Status;
use crate::APP_CONF;

static PUSHOVER_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

static PUSHOVER_API_URL: &'static str = "https://api.pushover.net/1/messages.json";

pub struct PushoverNotifier;

#[derive(Debug, thiserror::Error)]
#[error("Failed to deliver message to user keys: {0:?}")]
pub struct Error(Vec<(String, reqwest::Error)>);

impl Notifier for PushoverNotifier {
    type Config = notify::Pushover;
    type Error = Error;

    fn attempt(
        pushover: &Self::Config,
        notification: &Notification<'_>,
    ) -> Result<(), Self::Error> {
        // Build up the message text
        let mut message = String::new();

        if notification.startup == true {
            message.push_str("<b><i>This is a startup alert.</i></b>\n\n");
        } else if notification.changed == false {
            message.push_str("<b><i>This is a reminder.</i></b>\n\n");
        }

        message.push_str(&format!(
            "<u>Status:</u> <b><font color=\"{}\">{}</font></b>\n",
            status_to_color(&notification.status),
            notification.status.as_str().to_uppercase()
        ));
        message.push_str(&format!(
            "<u>Nodes:</u> {}\n",
            &notification.replicas.join(", ")
        ));
        message.push_str(&format!("<u>Time:</u> {}", &notification.time));

        tracing::debug!("will send Pushover notification with message: {}", &message);

        let mut failures = vec![];

        for user_key in &pushover.user_keys {
            // Build form parameters
            let mut params: HashMap<&str, &str> = HashMap::new();

            // Append authorization values
            params.insert("token", &pushover.app_token);
            params.insert("user", user_key);

            // Append title & message
            params.insert("title", &APP_CONF.branding.page_title);
            params.insert("message", &message);
            params.insert("html", "1");

            // Append target URL
            let url_title = format!("Details on {}", APP_CONF.branding.page_title);

            params.insert("url_title", &url_title);
            params.insert("url", APP_CONF.branding.page_url.as_str());

            // Mark as high-priority? (reminder)
            if notification.changed == false {
                params.insert("priority", "1");
            }

            // Submit message to Pushover
            let response = PUSHOVER_HTTP_CLIENT
                .post(PUSHOVER_API_URL)
                .form(&params)
                .send();

            // Check for any failure
            match response {
                Ok(response) => match response.error_for_status() {
                    Ok(_) => {}
                    Err(err) => {
                        failures.push((user_key.to_string(), err));
                    }
                },
                Err(err) => {
                    failures.push((user_key.to_string(), err));
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
        "pushover"
    }
}

fn status_to_color(status: &Status) -> &'static str {
    match status {
        &Status::Healthy => "#54A158",
        &Status::Sick => "#D5A048",
        &Status::Dead => "#C4291C",
    }
}
