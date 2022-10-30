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

mod defaults;

pub mod logger;
pub mod notify;
pub mod plugins;
pub mod probe;

use std::path::PathBuf;
use std::{net::SocketAddr, path::Path};

use config::{ConfigError, Environment, File};
use serde::Deserialize;
use url::Url;

use self::{notify::Notify, plugins::Plugins, probe::Probe};

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub assets: Assets,
    pub branding: Branding,
    pub metrics: Metrics,
    #[serde(default)]
    pub plugins: Plugins,
    pub notify: Notify,
    pub probe: Probe,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self, ConfigError> {
        let s = config::Config::builder()
            .add_source(File::from(path))
            .add_source(Environment::with_prefix("overvakt"))
            .build()?;

        s.try_deserialize()
    }
}

#[derive(Deserialize)]
pub struct Server {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,

    #[serde(default = "defaults::server_inet")]
    pub inet: SocketAddr,

    #[serde(default = "defaults::server_workers")]
    pub workers: usize,

    pub manager_token: String,
    pub reporter_token: String,
}

#[derive(Deserialize)]
pub struct Assets {
    #[serde(default = "defaults::assets_path")]
    pub path: PathBuf,
}

#[derive(Deserialize)]
pub struct Branding {
    #[serde(default = "defaults::branding_page_title")]
    pub page_title: String,

    pub page_url: Url,
    pub company_name: String,
    pub icon_color: String,
    pub icon_url: Url,
    pub logo_color: String,
    pub logo_url: Url,
    pub website_url: Url,
    pub support_url: Url,
    pub custom_html: Option<String>,
}

#[derive(Deserialize)]
pub struct Metrics {
    #[serde(default = "defaults::metrics_poll_interval")]
    pub poll_interval: u64,

    #[serde(default = "defaults::metrics_poll_retry")]
    pub poll_retry: u64,

    #[serde(default = "defaults::metrics_poll_http_status_healthy_above")]
    pub poll_http_status_healthy_above: u16,

    #[serde(default = "defaults::metrics_poll_http_status_healthy_below")]
    pub poll_http_status_healthy_below: u16,

    #[serde(default = "defaults::metrics_poll_delay_dead")]
    pub poll_delay_dead: u64,

    #[serde(default = "defaults::metrics_poll_delay_sick")]
    pub poll_delay_sick: u64,

    #[serde(default = "defaults::poll_parallelism")]
    pub poll_parallelism: u16,

    #[serde(default = "defaults::metrics_push_delay_dead")]
    pub push_delay_dead: u64,

    #[serde(default = "defaults::metrics_push_system_cpu_sick_above")]
    pub push_system_cpu_sick_above: f32,

    #[serde(default = "defaults::metrics_push_system_ram_sick_above")]
    pub push_system_ram_sick_above: f32,

    #[serde(default = "defaults::metrics_script_interval")]
    pub script_interval: u64,

    #[serde(default = "defaults::script_parallelism")]
    pub script_parallelism: u16,

    #[serde(default = "defaults::metrics_local_delay_dead")]
    pub local_delay_dead: u64,
}
