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
// Copyright: 2021, Valerian Saliou <valerian@valeriansaliou.name>
// Copyright: 2021, Bastien Orivel <eijebong@bananium.fr>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Serialize;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::config::notify;
use crate::prober::status::Status;
use crate::APP_CONF;

static ZULIP_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct ZulipNotifier;

#[derive(Serialize)]
struct ZulipPayload<'a> {
    #[serde(rename(serialize = "type"))]
    type_: &'a str,
    to: &'a str,
    topic: &'a str,
    content: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for ZulipNotifier {
    type Config = notify::Zulip;
    type Error = Error;

    fn attempt(zulip: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        let status_label = format!("{:?}", notification.status);

        let status_text = match notification.status {
            Status::Dead => " *dead* :boom:",
            Status::Healthy => " *healthy* :check_mark:",
            Status::Sick => " *sick* :sick:",
        };

        // Build message
        let mut message_text = if notification.startup == true {
            format!("Status started up, as: {}.", status_text)
        } else if notification.changed {
            format!("Status changed to: {}.", status_text)
        } else {
            format!("Status is still: {}.", status_text)
        };

        if notification.replicas.len() > 0 {
            let nodes_label = notification.replicas.join(", ");
            let nodes_label_titled = format!("\n **Nodes**: *{}*.", nodes_label);

            message_text.push_str(&nodes_label_titled);
        }

        message_text.push_str(&format!("\n **Status**: {}", &status_label));
        message_text.push_str(&format!("\n **Time**: {}", &notification.time));
        message_text.push_str(&format!(
            "\n **Page**: {}",
            &APP_CONF.branding.page_url.as_str()
        ));

        // Submit payload to Zulip
        let payload = ZulipPayload {
            type_: "stream",
            to: &zulip.channel,
            topic: "Övervakt status",
            content: &message_text,
        };

        let response = ZULIP_HTTP_CLIENT
            .post(zulip.api_url.join("messages").unwrap().as_str())
            .basic_auth(zulip.bot_email.clone(), Some(zulip.bot_api_key.clone()))
            .form(&payload)
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
        "zulip"
    }
}
