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
// Copyright: 2020, Rachel Chen <rachel@chens.email>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use super::generic::{Notification, Notifier, DISPATCH_TIMEOUT_SECONDS};
use crate::config::notify;
use crate::APP_CONF;

static GOTIFY_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(DISPATCH_TIMEOUT_SECONDS))
        .gzip(true)
        .build()
        .unwrap()
});

pub struct GotifyNotifier;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An HTTP error was returned.")]
    Http(#[from] reqwest::Error),

    #[error("Status was not success.")]
    NonSuccess(#[source] reqwest::Error),
}

impl Notifier for GotifyNotifier {
    type Config = notify::Gotify;
    type Error = Error;

    fn attempt(gotify: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error> {
        // Build up the message text
        let mut message = String::new();

        if notification.startup == true {
            message.push_str("This is a startup alert.\n\n");
        } else if notification.changed == false {
            message.push_str("This is a reminder.\n\n");
        }

        message.push_str(&format!(
            "Status: {}\n",
            notification.status.as_str().to_uppercase()
        ));
        message.push_str(&format!("Nodes:\n{}\n", &notification.replicas.join("\n")));
        message.push_str(&format!("Time: {}", &notification.time));

        log::debug!("will send Gotify notification with message: {}", &message);

        // Generate URL
        // See: https://gotify.net/docs/pushmsg
        let url = format!(
            "{}message?token={}",
            gotify.app_url.as_str(),
            gotify.app_token
        );

        // Build message parameters
        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("title", &APP_CONF.branding.page_title);
        params.insert("message", &message);

        if notification.changed == false {
            params.insert("priority", "10");
        }

        // Submit message to Gotify
        let response = GOTIFY_HTTP_CLIENT.post(&url).form(&params).send();

        match response {
            Ok(response) => match response.error_for_status() {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::NonSuccess(err)),
            },
            Err(err) => Err(Error::Http(err)),
        }
    }

    fn can_notify(gotify: &Self::Config, notification: &Notification<'_>) -> bool {
        notification.expected(gotify.reminders_only)
    }

    fn name() -> &'static str {
        "gotify"
    }
}
