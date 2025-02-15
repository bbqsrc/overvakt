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
mod util;

use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{process, thread};

use clap::{value_parser, Arg, Command};
use once_cell::sync::Lazy;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::aggregator::manager::run as run_aggregator;
use crate::config::Config;
use crate::prober::manager::{
    initialize_store as initialize_store_prober, run_poll as run_poll_prober,
    run_script as run_script_prober,
};

struct AppArgs {
    config: PathBuf,
}

pub static THREAD_NAME_PROBER_POLL: &str = "overvakt-prober-poll";
pub static THREAD_NAME_PROBER_SCRIPT: &str = "overvakt-prober-script";
pub static THREAD_NAME_AGGREGATOR: &str = "overvakt-aggregator";
pub static THREAD_NAME_RESPONDER: &str = "overvakt-responder";

macro_rules! gen_spawn_managed {
    ($name:expr, $method:ident, $thread_name:ident, $managed_fn:ident) => {
        fn $method() {
            tracing::debug!("spawn managed thread: {}", $name);

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
            if has_error {
                tracing::error!("managed thread crashed ({}), setting it up again", $name);

                // Prevents thread start loop floods
                thread::sleep(Duration::from_secs(1));

                $method();
            }
        }
    };
}

static APP_ARGS: Lazy<AppArgs> = Lazy::new(make_app_args);
static APP_CONF: Lazy<Config> = Lazy::new(|| {
    let c = match Config::new(&APP_ARGS.config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[!] Error loading config:");
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

    // Ensure assets path exists
    if !c.assets.path.exists() {
        eprintln!("assets directory not found: {:?}", c.assets.path);
        process::exit(1);
    }

    c
});

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
                .default_value("./overvakt.toml")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: matches
            .get_one::<PathBuf>("config")
            .expect("invalid config value")
            .clone(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize shared logger
    let env_filter = EnvFilter::default().add_directive(
        LevelFilter::from_str(&APP_CONF.server.log_level)
            .expect("invalid log level")
            .into(),
    );

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    tracing::info!("starting up");

    // Initialize prober store
    initialize_store_prober();

    // Spawn probes (background thread)
    thread::spawn(spawn_poll_prober);
    thread::spawn(spawn_script_prober);

    // Spawn aggregator (background thread)
    thread::spawn(spawn_aggregator);

    // Spawn Web responder (foreground thread)
    responder::manager::run().await?;

    tracing::info!("shutting down server");
    Ok(())
}
