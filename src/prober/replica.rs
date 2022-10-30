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

use serde::Serialize;
use url::{Host, Url};

#[derive(Serialize, Debug, Clone)]
pub enum ReplicaUrl {
    Icmp(String),
    Tcp(String, u16),
    Http(String),
    Https(String),
}

impl ReplicaUrl {
    pub fn parse_from(raw_url: &str) -> Result<ReplicaUrl, ()> {
        match Url::parse(raw_url) {
            Ok(url) => match url.scheme() {
                "icmp" => match (url.host(), url.port(), url.path_segments()) {
                    (Some(host), None, None) => Ok(ReplicaUrl::Icmp(Self::host_string(host))),
                    _ => Err(()),
                },
                "tcp" => match (url.host(), url.port(), url.path_segments()) {
                    (Some(host), Some(port), None) => {
                        Ok(ReplicaUrl::Tcp(Self::host_string(host), port))
                    }
                    _ => Err(()),
                },
                "http" => Ok(ReplicaUrl::Http(url.into())),
                "https" => Ok(ReplicaUrl::Https(url.into())),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn host_string(host: Host<&str>) -> String {
        // Convert internal host value into string. This is especially useful for IPv6 addresses, \
        //   which we need returned in '::1' format; as they would otherwise be returned in \
        //   '[::1]' format using built-in top-level 'to_string()' method on the 'Host' trait. The \
        //   underlying address parser does not accept IPv6 addresses formatted as '[::1]', so \
        //   this seemingly overkill processing is obviously needed.
        match host {
            Host::Domain(domain_value) => domain_value.to_string(),
            Host::Ipv4(ipv4_value) => ipv4_value.to_string(),
            Host::Ipv6(ipv6_value) => ipv6_value.to_string(),
        }
    }
}
