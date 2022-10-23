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

use std::fmt;
use std::ops::Deref;

use regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Regex(regex::Regex);

impl Deref for Regex {
    type Target = regex::Regex;

    fn deref(&self) -> &regex::Regex {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(de: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegexVisitor;

        impl<'de> Visitor<'de> for RegexVisitor {
            type Value = Regex;

            fn expecting(&self, format: &mut fmt::Formatter) -> fmt::Result {
                format.write_str("a regular expression pattern")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Regex, E> {
                regex::Regex::new(value)
                    .map(Regex)
                    .map_err(|err| E::custom(err.to_string()))
            }
        }

        de.deserialize_str(RegexVisitor)
    }
}

impl Serialize for Regex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Notice: ignore Regex serialization here, as it is not used in templates (which \
        //   serialization is used for in Övervakt).
        serializer.serialize_none()
    }
}
