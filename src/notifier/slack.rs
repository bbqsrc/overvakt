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

use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Serialize;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::config::notify;
use crate::prober::status::Status;
use crate::APP_CONF;

static SLACK_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct SlackNotifier;

#[derive(Serialize)]
struct SlackPayload<'a> {
    text: String,
    attachments: Vec<SlackPayloadAttachment<'a>>,
}

#[derive(Serialize)]
struct SlackPayloadAttachment<'a> {
    fallback: String,
    color: &'a str,
    fields: Vec<SlackPayloadAttachmentField<'a>>,
}

#[derive(Serialize)]
struct SlackPayloadAttachmentField<'a> {
    title: &'a str,
    value: &'a str,
    short: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for SlackNotifier {
    type Config = notify::Slack;
    type Error = Error;

    fn attempt(slack: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        let status_label = format!("{:?}", notification.status);
        let mut nodes_label = String::new();

        // Build message
        let message_text = if notification.startup {
            format!("Status started up, as: *{}*.", notification.status.as_str())
        } else if notification.changed {
            format!("Status changed to: *{}*.", notification.status.as_str())
        } else {
            format!("Status is still: *{}*.", notification.status.as_str())
        };

        let payload_text = if slack.mention_channel {
            format!("<!channel> {}", &message_text)
        } else {
            message_text.clone()
        };

        // Build paylaod
        let mut payload = SlackPayload {
            text: payload_text,
            attachments: Vec::new(),
        };

        let mut attachment = SlackPayloadAttachment {
            fallback: message_text,
            color: status_to_color(notification.status),
            fields: Vec::new(),
        };

        // Append attachment fields
        if !notification.replicas.is_empty() {
            nodes_label.push_str(&notification.replicas.join(", "));

            let nodes_label_titled = format!(" Nodes: *{}*.", nodes_label);

            payload.text.push_str(&nodes_label_titled);
            attachment.fallback.push_str(&nodes_label_titled);

            attachment.fields.push(SlackPayloadAttachmentField {
                title: "Nodes",
                value: &nodes_label,
                short: false,
            });
        }

        attachment.fields.push(SlackPayloadAttachmentField {
            title: "Status",
            value: &status_label,
            short: true,
        });

        attachment.fields.push(SlackPayloadAttachmentField {
            title: "Time",
            value: &notification.time,
            short: true,
        });

        attachment.fields.push(SlackPayloadAttachmentField {
            title: "Monitor Page",
            value: APP_CONF.branding.page_url.as_str(),
            short: false,
        });

        // Append attachment
        payload.attachments.push(attachment);

        // Submit payload to Slack
        let response = SLACK_HTTP_CLIENT
            .post(slack.hook_url.as_str())
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
        "slack"
    }
}

fn status_to_color(status: &Status) -> &'static str {
    match status {
        Status::Healthy => "good",
        Status::Sick => "warning",
        Status::Dead => "danger",
    }
}
