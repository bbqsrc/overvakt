use serde::{Deserialize, Serialize};
use url::Url;

use super::defaults;

#[derive(Deserialize)]
pub struct Notify {
    #[serde(default = "defaults::notify_startup_notification")]
    pub startup_notification: bool,

    pub reminder_interval: Option<u64>,

    #[serde(default)]
    pub reminder_backoff_function: ReminderBackoffFunction,

    #[serde(default = "defaults::notify_reminder_backoff_limit")]
    pub reminder_backoff_limit: u16,

    pub email: Option<Email>,
    pub twilio: Option<Twilio>,
    pub slack: Option<Slack>,
    pub zulip: Option<Zulip>,
    pub telegram: Option<Telegram>,
    pub pushover: Option<Pushover>,
    pub gotify: Option<Gotify>,
    pub xmpp: Option<Xmpp>,
    pub matrix: Option<Matrix>,
    pub webex: Option<WebEx>,
    pub webhook: Option<WebHook>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderBackoffFunction {
    #[serde(rename = "none")]
    None = 0,

    #[serde(rename = "linear")]
    Linear = 1,

    #[serde(rename = "square")]
    Square = 2,

    #[serde(rename = "cubic")]
    Cubic = 3,
}

impl Default for ReminderBackoffFunction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Deserialize)]
pub struct Email {
    pub to: String,
    pub from: String,

    #[serde(default = "defaults::notify_email_smtp_host")]
    pub smtp_host: String,

    #[serde(default = "defaults::notify_email_smtp_port")]
    pub smtp_port: u16,

    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,

    #[serde(default = "defaults::notify_email_smtp_encrypt")]
    pub smtp_encrypt: bool,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Twilio {
    pub to: Vec<String>,
    pub service_sid: String,
    pub account_sid: String,
    pub auth_token: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Slack {
    pub hook_url: Url,

    #[serde(default = "defaults::notify_slack_mention_channel")]
    pub mention_channel: bool,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Zulip {
    pub bot_email: String,
    pub bot_api_key: String,
    pub channel: String,
    pub api_url: Url,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Telegram {
    pub bot_token: String,
    pub chat_id: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Pushover {
    pub app_token: String,
    pub user_keys: Vec<String>,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Gotify {
    pub app_url: Url,
    pub app_token: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Xmpp {
    pub to: String,
    pub from: String,
    pub xmpp_password: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct Matrix {
    pub homeserver_url: Url,
    pub access_token: String,
    pub room_id: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct WebEx {
    pub endpoint_url: Url,
    pub token: String,
    pub room_id: String,

    #[serde(default = "defaults::notify_generic_reminders_only")]
    pub reminders_only: bool,
}

#[derive(Deserialize)]
pub struct WebHook {
    pub hook_url: Url,
}
