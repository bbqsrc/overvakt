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

use super::generic::{GenericNotifier, Notification, DISPATCH_TIMEOUT_SECONDS};
use crate::config::config::ConfigNotify;
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

impl GenericNotifier for SlackNotifier {
    type Config = ConfigNotify;
    type Error = bool;

    fn attempt(notify: &ConfigNotify, notification: &Notification<'_>) -> Result<(), bool> {
        if let Some(ref slack) = notify.slack {
            let status_label = format!("{:?}", notification.status);
            let mut nodes_label = String::new();

            // Build message
            let message_text = if notification.startup == true {
                format!("Status started up, as: *{}*.", notification.status.as_str())
            } else if notification.changed == true {
                format!("Status changed to: *{}*.", notification.status.as_str())
            } else {
                format!("Status is still: *{}*.", notification.status.as_str())
            };

            let payload_text = if slack.mention_channel == true {
                format!("<!channel> {}", &message_text)
            } else {
                message_text.to_owned()
            };

            // Build paylaod
            let mut payload = SlackPayload {
                text: payload_text,
                attachments: Vec::new(),
            };

            let mut attachment = SlackPayloadAttachment {
                fallback: message_text,
                color: status_to_color(&notification.status),
                fields: Vec::new(),
            };

            // Append attachment fields
            if notification.replicas.len() > 0 {
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
        if let Some(ref slack_config) = notify.slack {
            notification.expected(slack_config.reminders_only)
        } else {
            false
        }
    }

    fn name() -> &'static str {
        "slack"
    }
}

fn status_to_color(status: &Status) -> &'static str {
    match status {
        &Status::Healthy => "good",
        &Status::Sick => "warning",
        &Status::Dead => "danger",
    }
}
