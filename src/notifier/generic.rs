// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::thread;
use std::time::Duration;

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

pub trait GenericNotifier {
    type Config;
    type Error;

    fn attempt(notify: &Self::Config, notification: &Notification) -> Result<(), Self::Error>;
    fn can_notify(notify: &Self::Config, notification: &Notification) -> bool;
    fn name() -> &'static str;
}

impl<'a> Notification<'a> {
    pub fn dispatch<N: GenericNotifier>(
        notify: &N::Config,
        notification: &Notification,
    ) -> Result<(), Vec<N::Error>> {
        if N::can_notify(notify, notification) == true {
            info!(
                "dispatch {} notification for status: {:?} and replicas: {:?}",
                N::name(),
                notification.status,
                notification.replicas
            );

            let mut errors = vec![];
            for try_index in 1..(DISPATCH_TRY_ATTEMPT_TIMES + 1) {
                debug!(
                    "dispatch {} notification attempt: #{}",
                    N::name(),
                    try_index
                );

                // Hold on for next try
                if try_index > 1 {
                    thread::sleep(Duration::from_secs(DISPATCH_TRY_WAIT_SECONDS))
                }

                // Attempt notification dispatch
                match N::attempt(notify, notification) {
                    Ok(_) => {
                        debug!("dispatched notification to provider: {}", N::name());
                        return Ok(());
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }

            error!("failed dispatching notification to provider: {}", N::name());
            return Err(errors);
        }

        debug!("did not dispatch notification to provider: {}", N::name());

        Ok(())
    }

    pub fn expected(&self, reminders_only: bool) -> bool {
        // Notification may not be expected if status has changed, but we only want to receive \
        //   reminders on this specific notifier channel.
        if reminders_only == false || (reminders_only == true && self.changed == false) {
            true
        } else {
            false
        }
    }
}
