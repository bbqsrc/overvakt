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

use super::generic::{GenericNotifier, Notification, DISPATCH_TIMEOUT_SECONDS};
use crate::config::config::ConfigNotify;
use crate::APP_CONF;

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

impl GenericNotifier for WebExNotifier {
    type Config = ConfigNotify;
    type Error = bool;

    fn attempt(notify: &ConfigNotify, notification: &Notification<'_>) -> Result<(), bool> {
        if let Some(ref webex) = notify.webex {
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
                .header("Authorization", "Bearer ".to_owned() + webex.token.as_str())
                .json(&payload)
                .send();

            if let Ok(response_inner) = response {
                if response_inner.status().is_success() == true {
                    return Ok(());
                }
            }

            return Err(true);
        }

        Err(false)
    }

    fn can_notify(notify: &ConfigNotify, notification: &Notification<'_>) -> bool {
        if let Some(ref webex_config) = notify.webex {
            notification.expected(webex_config.reminders_only)
        } else {
            false
        }
    }

    fn name() -> &'static str {
        "webex"
    }
}
