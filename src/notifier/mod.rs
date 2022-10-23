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

pub mod generic;

#[cfg(feature = "notifier-email")]
pub mod email;

#[cfg(feature = "notifier-twilio")]
pub mod twilio;

#[cfg(feature = "notifier-slack")]
pub mod slack;

#[cfg(feature = "notifier-zulip")]
pub mod zulip;

#[cfg(feature = "notifier-telegram")]
pub mod telegram;

#[cfg(feature = "notifier-pushover")]
pub mod pushover;

#[cfg(feature = "notifier-gotify")]
pub mod gotify;

#[cfg(feature = "notifier-xmpp")]
pub mod xmpp;

#[cfg(feature = "notifier-matrix")]
pub mod matrix;

#[cfg(feature = "notifier-webex")]
pub mod webex;

#[cfg(feature = "notifier-webhook")]
pub mod webhook;
