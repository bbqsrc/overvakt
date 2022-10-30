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
// Copyright: 2021, Enrico Risa https://github.com/wolf4ood
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::{config::notify, APP_CONF};

static MATRIX_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});
static MATRIX_FORMATTERS: Lazy<Vec<fn(&Notification<'_>) -> String>> = Lazy::new(|| {
    vec![
        format_status,
        format_replicas,
        format_status_page,
        format_time,
    ]
});

static MATRIX_MESSAGE_BODY: &'static str = "You received a Övervakt alert.";
static MATRIX_MESSAGE_TYPE: &'static str = "m.text";
static MATRIX_MESSAGE_FORMAT: &'static str = "org.matrix.custom.html";

pub struct MatrixNotifier;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for MatrixNotifier {
    type Config = notify::Matrix;
    type Error = Error;

    fn attempt(matrix: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        // Build up the message text
        let message = format_message(notification);

        log::debug!("will send Matrix notification with message: {}", &message);

        // Generate URL
        // See: https://matrix.org/docs/guides/client-server-api#sending-messages
        let url = format!(
            "{}_matrix/client/r0/rooms/{}/send/m.room.message?access_token={}",
            matrix.homeserver_url.as_str(),
            matrix.room_id.as_str(),
            matrix.access_token.as_str()
        );

        // Build message parameters
        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("body", MATRIX_MESSAGE_BODY);
        params.insert("msgtype", MATRIX_MESSAGE_TYPE);
        params.insert("format", MATRIX_MESSAGE_FORMAT);
        params.insert("formatted_body", &message);

        // Submit message to Matrix
        let response = MATRIX_HTTP_CLIENT.post(&url).json(&params).send();

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
        "matrix"
    }
}

fn format_status(notification: &Notification<'_>) -> String {
    let msg = if notification.startup == true {
        "Status started up, as"
    } else if notification.changed == true {
        "Status changed to"
    } else {
        "Status is still"
    };

    format!(
        "<p>{} {}: <em>{}</em>.</p>",
        notification.status.as_icon(),
        msg,
        notification.status.as_str().to_uppercase()
    )
}

fn format_replicas(notification: &Notification<'_>) -> String {
    let replicas = notification
        .replicas
        .iter()
        .map(|replica| replica.split(":").take(2).collect::<Vec<&str>>().join(":"))
        .fold(HashMap::new(), |mut replicas_count, replica| {
            *replicas_count.entry(replica).or_insert(0) += 1;
            replicas_count
        })
        .iter()
        .map(|(service_and_node, count)| {
            format!(
                "<li><code>{}</code>: {} {}</li>",
                service_and_node,
                count,
                notification.status.as_str()
            )
        })
        .collect::<Vec<String>>();

    if replicas.is_empty() {
        "".to_string()
    } else {
        format!("<ul>{}</ul>", replicas.join(""))
    }
}

fn format_status_page(_: &Notification<'_>) -> String {
    format!(
        "<p>Status page: {}</p>",
        APP_CONF.branding.page_url.as_str()
    )
}

fn format_time(notification: &Notification<'_>) -> String {
    format!("<p>Time: {}</p>", notification.time)
}

fn format_message(notification: &Notification<'_>) -> String {
    MATRIX_FORMATTERS
        .iter()
        .fold(String::new(), |mut accumulator, formatter| {
            accumulator.push_str(formatter(notification).as_str());
            accumulator
        })
}
