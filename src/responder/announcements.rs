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
// Copyright: 2022, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::Serialize;

pub static STORE: Lazy<Arc<RwLock<Store>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Store {
        announcements: Vec::new(),
    }))
});

pub struct Store {
    pub announcements: Vec<Announcement>,
}

#[derive(Serialize)]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub text: String,
    pub date: Option<String>,
}
