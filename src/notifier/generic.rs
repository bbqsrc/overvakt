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

use std::thread;
use std::time::Duration;

use crate::notifier::Error;
use crate::prober::status::Status;

const DISPATCH_TRY_WAIT_SECONDS: u64 = 2;
const DISPATCH_TRY_ATTEMPT_TIMES: u8 = 3;
pub const DISPATCH_TIMEOUT_SECONDS: u64 = 10;

pub struct Notification<'a> {
    pub status: &'a Status,
    pub time: String,
    pub replicas: Vec<&'a str>,
    pub changed: bool,
    pub startup: bool,
}

pub trait Notifier {
    type Config;
    type Error: std::error::Error + Send + Sync + 'static;

    fn attempt(notify: &Self::Config, notification: &Notification<'_>) -> Result<(), Self::Error>;
    fn can_notify(notify: &Self::Config, notification: &Notification<'_>) -> bool;
    fn name() -> &'static str;
}

impl<'a> Notification<'a> {
    pub fn dispatch<N: Notifier>(
        notify: &N::Config,
        notification: &Notification<'_>,
    ) -> Result<(), Error> {
        if N::can_notify(notify, notification) {
            tracing::info!(
                "dispatch {} notification for status: {:?} and replicas: {:?}",
                N::name(),
                notification.status,
                notification.replicas
            );

            let mut errors = vec![];
            for try_index in 1..=DISPATCH_TRY_ATTEMPT_TIMES {
                tracing::debug!(
                    "dispatch {} notification attempt: #{}",
                    N::name(),
                    try_index
                );

                // Hold on for next try
                if try_index > 1 {
                    thread::sleep(Duration::from_secs(DISPATCH_TRY_WAIT_SECONDS));
                }

                // Attempt notification dispatch
                match N::attempt(notify, notification) {
                    Ok(_) => {
                        tracing::debug!("dispatched notification to provider: {}", N::name());
                        return Ok(());
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }

            tracing::error!("failed dispatching notification to provider: {}", N::name());

            return Err(Error {
                name: N::name(),
                errors: errors.into_iter().map(Into::into).collect(),
            });
        }

        tracing::debug!("did not dispatch notification to provider: {}", N::name());

        Ok(())
    }

    pub fn expected(&self, reminders_only: bool) -> bool {
        // Notification may not be expected if status has changed, but we only want to receive \
        //   reminders on this specific notifier channel.
        !reminders_only || !self.changed
    }
}
