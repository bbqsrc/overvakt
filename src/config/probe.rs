use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::prober::mode::Mode;

#[derive(Deserialize)]
pub struct Probe {
    #[serde(default)]
    pub service: IndexMap<String, Service>,
}

#[derive(Deserialize)]
pub struct Service {
    pub label: String,
    #[serde(default)]
    pub node: IndexMap<String, ServiceNode>,
}

#[derive(Deserialize)]
pub struct ServiceNode {
    pub label: String,
    pub mode: Mode,
    pub replicas: Option<Vec<String>>,
    pub scripts: Option<Vec<String>>,
    #[serde(default)]
    pub http_no_cache_buster: bool,
    #[serde(default)]
    #[serde(with = "http_serde::header_map")]
    pub http_headers: http::HeaderMap,
    pub http_method: Option<HttpMethod>,
    pub http_body: Option<String>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    pub http_body_healthy_match: Option<Regex>,
    pub rabbitmq_queue: Option<String>,
    pub rabbitmq_queue_nack_healthy_below: Option<u32>,
    pub rabbitmq_queue_nack_dead_above: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Head,
    Get,
    Post,
    Put,
    Patch,
}
