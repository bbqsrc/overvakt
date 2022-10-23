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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Mode {
    #[serde(rename = "poll")]
    Poll,

    #[serde(rename = "push")]
    Push,

    #[serde(rename = "script")]
    Script,

    #[serde(rename = "local")]
    Local,
}
