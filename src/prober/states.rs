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

use std::time::{Duration, SystemTime};

use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::mode::Mode;
use super::replica::ReplicaURL;
use super::status::Status;
use crate::config::config::ConfigProbeServiceNodeHTTPMethod;

#[derive(Serialize)]
pub struct ServiceStates {
    pub status: Status,
    pub date: Option<String>,
    pub probes: IndexMap<String, ServiceStatesProbe>,
    pub notifier: ServiceStatesNotifier,
}

#[derive(Serialize)]
pub struct ServiceStatesProbe {
    pub id: String,
    pub label: String,
    pub status: Status,
    pub nodes: IndexMap<String, ServiceStatesProbeNode>,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    Raw,
    Dgram,
}

impl Default for SocketType {
    fn default() -> Self {
        Self::Raw
    }
}

impl From<SocketType> for socket2::Type {
    fn from(ty: SocketType) -> Self {
        match ty {
            SocketType::Raw => socket2::Type::RAW,
            SocketType::Dgram => socket2::Type::DGRAM,
        }
    }
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNode {
    pub status: Status,
    pub label: String,
    pub mode: Mode,
    pub replicas: IndexMap<String, ServiceStatesProbeNodeReplica>,
    #[serde(default)]
    #[serde(with = "http_serde::header_map")]
    pub http_headers: http::HeaderMap,
    pub http_method: Option<ConfigProbeServiceNodeHTTPMethod>,
    pub http_body: Option<String>,
    #[serde(with = "serde_regex")]
    pub http_body_healthy_match: Option<Regex>,
    pub rabbitmq: Option<ServiceStatesProbeNodeRabbitMQ>,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplica {
    pub status: Status,
    pub url: Option<ReplicaURL>,
    pub script: Option<String>,
    pub metrics: ServiceStatesProbeNodeReplicaMetrics,
    pub load: Option<ServiceStatesProbeNodeReplicaLoad>,
    pub report: Option<ServiceStatesProbeNodeReplicaReport>,
}

#[derive(Serialize, Clone)]
pub struct ServiceStatesProbeNodeRabbitMQ {
    pub queue: String,
    pub queue_nack_healthy_below: Option<u32>,
    pub queue_nack_dead_above: Option<u32>,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaMetrics {
    pub latency: Option<u64>,
    pub system: Option<ServiceStatesProbeNodeReplicaMetricsSystem>,
    pub rabbitmq: Option<ServiceStatesProbeNodeReplicaMetricsRabbitMQ>,
}

#[derive(Serialize, Clone)]
pub struct ServiceStatesProbeNodeReplicaMetricsSystem {
    pub cpu: u16,
    pub ram: u16,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaMetricsRabbitMQ {
    pub queue_ready: u32,
    pub queue_nack: u32,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplicaLoad {
    pub cpu: f32,
    pub ram: f32,
    pub queue: ServiceStatesProbeNodeReplicaLoadQueue,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaLoadQueue {
    pub loaded: bool,
    pub stalled: bool,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplicaReport {
    pub time: SystemTime,
    pub interval: Duration,
}

#[derive(Serialize)]
pub struct ServiceStatesNotifier {
    pub reminder_backoff_counter: u16,
    pub reminder_ignore_until: Option<SystemTime>,
}
