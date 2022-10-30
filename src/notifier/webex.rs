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
// Copyright: 2022, Valerian Saliou <valerian@valeriansaliou.name>
// Copyright: 2022, Timmy O'Tool https://github.com/TimmyOtool
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Serialize;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::{config::notify, APP_CONF};

static WEBEX_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct WebExNotifier;

#[derive(Serialize)]
struct WebExPayload<'a> {
    #[serde(rename = "roomId")]
    room_id: &'a str,
    text: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for WebExNotifier {
    type Config = notify::WebEx;
    type Error = Error;

    fn attempt(webex: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        let nodes_label = notification.replicas.join(", ");

        // Build up the message text
        let mut message = String::new();

        if notification.startup {
            message.push_str(&format!(
                "Status startup alert from: {}\n",
                APP_CONF.branding.page_title
            ));
        } else if notification.changed {
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

        message.push_str(&format!("Status: {:?}\n", notification.status));
        message.push_str(&format!("Nodes: {}\n", &nodes_label));
        message.push_str(&format!("Time: {}\n", &notification.time));
        message.push_str(&format!("URL: {}", APP_CONF.branding.page_url.as_str()));

        // Build paylaod
        let payload = WebExPayload {
            room_id: webex.room_id.as_str(),
            text: &message,
        };

        // Submit payload to Webex
        let response = WEBEX_HTTP_CLIENT
            .post(webex.endpoint_url.as_str())
            .header("Authorization", format!("Bearer {}", &webex.token))
            .json(&payload)
            .send();

        match response {
            Ok(response) => match response.error_for_status() {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::NonSuccess(err)),
            },
            Err(err) => Err(Error::Http(err)),
        }
    }

    fn can_notify(config: &Self::Config, notification: &Notification<'_>) -> bool {
        notification.expected(config.reminders_only)
    }

    fn name() -> &'static str {
        "webex"
    }
}
