use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::prober::mode::Mode;

#[derive(Deserialize)]
pub struct Probe {
    pub service: Vec<Service>,
}

#[derive(Deserialize)]
pub struct Service {
    pub id: String,
    pub label: String,
    pub node: Vec<ServiceNode>,
}

#[derive(Deserialize)]
pub struct ServiceNode {
    pub id: String,
    pub label: String,
    pub mode: Mode,
    pub replicas: Option<Vec<String>>,
    pub scripts: Option<Vec<String>>,
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
