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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    #[serde(rename = "healthy")]
    Healthy,

    #[serde(rename = "sick")]
    Sick,

    #[serde(rename = "dead")]
    Dead,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Healthy => "healthy",
            Status::Sick => "sick",
            Status::Dead => "dead",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            Status::Dead => "\u{274c}",
            Status::Sick => "\u{26a0}",
            Status::Healthy => "\u{2705}",
        }
    }
}
