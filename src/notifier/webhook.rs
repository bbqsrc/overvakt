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

use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Serialize;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::config::notify;
use crate::prober::status::Status;
use crate::APP_CONF;

static WEBHOOK_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct WebHookNotifier;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

#[derive(Serialize)]
struct WebHookPayload<'a> {
    #[serde(rename = "type")]
    _type: WebHookPayloadType,

    status: &'a Status,
    time: &'a str,
    replicas: &'a [&'a str],
    page: WebHookPayloadPage<'a>,
}

#[derive(Serialize)]
pub enum WebHookPayloadType {
    #[serde(rename = "startup")]
    Startup,

    #[serde(rename = "changed")]
    Changed,

    #[serde(rename = "reminder")]
    Reminder,
}

#[derive(Serialize)]
struct WebHookPayloadPage<'a> {
    title: &'a str,
    url: &'a str,
}

impl Notifier for WebHookNotifier {
    type Config = notify::WebHook;
    type Error = Error;

    fn attempt(webhook: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        // Acquire hook type
        let hook_type = if notification.startup {
            WebHookPayloadType::Startup
        } else if notification.changed {
            WebHookPayloadType::Changed
        } else {
            WebHookPayloadType::Reminder
        };

        // Build paylaod
        let payload = WebHookPayload {
            _type: hook_type,
            status: notification.status,
            time: notification.time.as_str(),
            replicas: &notification.replicas,
            page: WebHookPayloadPage {
                title: APP_CONF.branding.page_title.as_str(),
                url: APP_CONF.branding.page_url.as_str(),
            },
        };

        // Submit payload to Web Hooks
        let response = WEBHOOK_HTTP_CLIENT
            .post(webhook.hook_url.as_str())
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

    fn can_notify(_: &Self::Config, _: &Notification<'_>) -> bool {
        true
    }

    fn name() -> &'static str {
        "webhook"
    }
}
