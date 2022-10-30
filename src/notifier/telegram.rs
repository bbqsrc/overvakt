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
use serde::Serialize;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::{config::notify, APP_CONF};

static TELEGRAM_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

static TELEGRAM_API_BASE_URL: &'static str = "https://api.telegram.org";

pub struct TelegramNotifier;

#[derive(Serialize)]
struct TelegramPayload<'a> {
    chat_id: TelegramChatID<'a>,
    text: String,
    parse_mode: &'static str,
    disable_web_page_preview: bool,
}

#[derive(Serialize)]
#[serde(untagged)]
enum TelegramChatID<'a> {
    Group(&'a str),
    User(u64),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for TelegramNotifier {
    type Config = notify::Telegram;
    type Error = Error;

    fn attempt(
        telegram: &Self::Config,
        notification: &Notification<'_>,
    ) -> Result<(), Self::Error> {
        // Build message
        let mut message = if notification.startup == true {
            format!(
                "{} Status started up, as: *{}*.\n",
                notification.status.as_icon(),
                notification.status.as_str().to_uppercase()
            )
        } else if notification.changed == true {
            format!(
                "{} Status changed to: *{}*.\n",
                notification.status.as_icon(),
                notification.status.as_str().to_uppercase()
            )
        } else {
            format!(
                "{} Status is still: *{}*.\n",
                notification.status.as_icon(),
                notification.status.as_str().to_uppercase()
            )
        };

        let mut replicas_count: HashMap<String, u32> = HashMap::new();

        for replica in notification.replicas.iter() {
            let service_and_node = replica.split(":").take(2).collect::<Vec<&str>>().join(":");
            *replicas_count.entry(service_and_node).or_insert(0) += 1;
        }

        let nodes_count_list_text = replicas_count
            .iter()
            .map(|(service_and_node, count)| {
                format!(
                    "- `{}`: {} {}",
                    service_and_node,
                    count,
                    notification.status.as_str()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        message.push_str(&nodes_count_list_text);
        message.push_str(&format!("\nLink: {}", APP_CONF.branding.page_url.as_str()));

        log::debug!("will send Telegram notification with message: {}", &message);

        // Generate Telegram chat identifier
        let chat_id = match &telegram.chat_id.parse::<u64>() {
            Ok(user_chat_id) => TelegramChatID::User(*user_chat_id),
            Err(_) => TelegramChatID::Group(&telegram.chat_id.as_str()),
        };

        // Build payload
        let payload = TelegramPayload {
            chat_id: chat_id,
            text: message,
            parse_mode: "markdown",
            disable_web_page_preview: true,
        };

        // Generate target API URL
        let url = format!(
            "{}/bot{}/sendMessage",
            TELEGRAM_API_BASE_URL, telegram.bot_token
        );

        // Submit message to Telegram
        let response = TELEGRAM_HTTP_CLIENT
            .post(url.as_str())
            .json(&payload)
            .send();

        // Check for any failure
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
        "telegram"
    }
}
