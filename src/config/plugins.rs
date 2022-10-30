use serde::Deserialize;
use url::Url;

use crate::prober::states::SocketType;

#[derive(Deserialize, Default)]
pub struct Plugins {
    pub rabbitmq: Option<PluginRabbitMQ>,
    pub icmp: Option<PluginIcmp>,
}

#[derive(Deserialize)]
pub struct PluginIcmp {
    #[serde(default)]
    pub socket_type: SocketType,
}

#[derive(Deserialize)]
pub struct PluginRabbitMQ {
    pub api_url: Url,
    pub auth_username: String,
    pub auth_password: String,
    pub virtualhost: String,
    pub queue_ready_healthy_below: u32,
    pub queue_nack_healthy_below: u32,
    pub queue_ready_dead_above: u32,
    pub queue_nack_dead_above: u32,
    pub queue_loaded_retry_delay: Option<u64>,
}
