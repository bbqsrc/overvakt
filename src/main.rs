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

#![deny(rust_2018_idioms)]

mod aggregator;
mod config;
mod notifier;
mod prober;
mod responder;

use std::ops::Deref;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use clap::{Arg, Command};
use log::LevelFilter;

use crate::aggregator::manager::run as run_aggregator;
use crate::config::config::Config;
use crate::config::logger::ConfigLogger;
use crate::config::reader::ConfigReader;
use crate::prober::manager::{
    initialize_store as initialize_store_prober, run_poll as run_poll_prober,
    run_script as run_script_prober,
};

struct AppArgs {
    config: String,
}

pub static THREAD_NAME_PROBER_POLL: &'static str = "overvakt-prober-poll";
pub static THREAD_NAME_PROBER_SCRIPT: &'static str = "overvakt-prober-script";
pub static THREAD_NAME_AGGREGATOR: &'static str = "overvakt-aggregator";
pub static THREAD_NAME_RESPONDER: &'static str = "overvakt-responder";

macro_rules! gen_spawn_managed {
    ($name:expr, $method:ident, $thread_name:ident, $managed_fn:ident) => {
        fn $method() {
            log::debug!("spawn managed thread: {}", $name);

            let worker = thread::Builder::new()
                .name($thread_name.to_string())
                .spawn($managed_fn);

            // Block on worker thread (join it)
            let has_error = if let Ok(worker_thread) = worker {
                worker_thread.join().is_err()
            } else {
                true
            };

            // Worker thread crashed?
            if has_error == true {
                log::error!("managed thread crashed ({}), setting it up again", $name);

                // Prevents thread start loop floods
                thread::sleep(Duration::from_secs(1));

                $method();
            }
        }
    };
}

lazy_static::lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_CONF: Config = ConfigReader::make();
}

gen_spawn_managed!(
    "prober-poll",
    spawn_poll_prober,
    THREAD_NAME_PROBER_POLL,
    run_poll_prober
);
gen_spawn_managed!(
    "prober-script",
    spawn_script_prober,
    THREAD_NAME_PROBER_SCRIPT,
    run_script_prober
);
gen_spawn_managed!(
    "aggregator",
    spawn_aggregator,
    THREAD_NAME_AGGREGATOR,
    run_aggregator
);

fn make_app_args() -> AppArgs {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to configuration file")
                .default_value("./overvakt.toml"),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: matches
            .get_one::<String>("config")
            .expect("invalid config value")
            .to_string(),
    }
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());

    // Ensure assets path exists
    assert_eq!(
        APP_CONF.assets.path.exists(),
        true,
        "assets directory not found: {:?}",
        APP_CONF.assets.path
    );
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize shared logger
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(&APP_CONF.server.log_level).expect("invalid log level"),
    );

    log::info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // Initialize prober store
    initialize_store_prober();

    // Spawn probes (background thread)
    thread::spawn(spawn_poll_prober);
    thread::spawn(spawn_script_prober);

    // Spawn aggregator (background thread)
    thread::spawn(spawn_aggregator);

    // Spawn Web responder (foreground thread)
    responder::manager::run().await?;

    log::info!("shutting down server");
    Ok(())
}
