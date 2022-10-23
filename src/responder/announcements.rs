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
use std::sync::RwLock;

use serde::Serialize;
use time;
use time::format_description::FormatItem;

lazy_static::lazy_static! {
    pub static ref STORE: Arc<RwLock<Store>> = Arc::new(RwLock::new(Store {
        announcements: Vec::new(),
    }));
    pub static ref DATE_NOW_FORMATTER: Vec<FormatItem<'static>> = time::format_description::parse(
        "[day padding:none] [month repr:short] [year], \
        [hour]:[minute]:[second] UTC[offset_hour sign:mandatory]:[offset_minute]"
    )
    .expect("invalid time format");
}

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
