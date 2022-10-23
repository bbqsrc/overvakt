Övervakt
========

[![Test and Build](https://github.com/bbqsrc/overvakt/workflows/Test%20and%20Build/badge.svg?branch=main)](https://github.com/bbqsrc/overvakt/actions?query=workflow%3A%22Test+and+Build%22) [![Build and Release](https://github.com/bbqsrc/overvakt/workflows/Build%20and%20Release/badge.svg)](https://github.com/bbqsrc/overvakt/actions?query=workflow%3A%22Build+and+Release%22) [![dependency status](https://deps.rs/repo/github/bbqsrc/overvakt/status.svg)](https://deps.rs/repo/github/bbqsrc/overvakt)

**Service uptime monitoring.**

__This codebase has recently been forked from Vigil and is expected to undergo an amount of churn before v1.0.__

Övervakt is an open-source status page you can host on your infrastructure, used to monitor all your servers and apps, and visible to your users (on a domain of your choice, eg. `status.example.com`).

It is useful in microservices contexts to monitor both apps and backends. If a node goes down in your infrastructure, you receive a status change notification in a Slack channel, Email, Twilio SMS and/or XMPP.

## Features

* **Monitors your infrastructure services automatically**
* **Notifies you when a service gets down** or gets back up via a configured channel:
  * Email
  * Twilio (SMS)
  * Slack
  * Zulip
  * Telegram
  * Pushover
  * Gotify
  * XMPP
  * Matrix
  * Cisco Webex
  * Webhook
* **Generates a status page**, that you can host on your domain for your public users (eg. `https://status.example.com`)
* **Allows publishing announcements**, eg. let your users know that a planned maintenance is upcoming

## How does it work?

Övervakt monitors all your infrastructure services. You first need to configure target services to be monitored, and then Övervakt does the rest for you.

**Övervakt can monitor:**

* **HTTP / TCP / ICMP services**: Övervakt frequently probes an HTTP, TCP or ICMP target and checks for reachability

It is recommended to configure Övervakt to send frequent probe checks, as to ensure you are quickly notified when a service gets down (thus to reduce unexpected downtime on your services).

## How to use it?

### Installation

<!--
Övervakt is built in Rust. To install it, either download a version from the [Övervakt releases](https://github.com/bbqsrc/overvakt/releases) page, use `cargo install` or pull the source code from `main`.

**Install from Cargo:**

If you prefer managing `overvakt` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install overvakt-server
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Övervakt using the `overvakt` command.
-->

**Install from source:**

The last option is to pull the source code from Git and compile Övervakt via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

<!-- 
_Install `libssl-dev` (ie. OpenSSL headers) and `libstrophe-dev` (ie. XMPP library headers; only if you need the XMPP notifier) before you compile Övervakt. SSL dependencies are required for the HTTPS probes and email notifications._

**Install from Docker Hub:**

You might find it convenient to run Övervakt via Docker. You can find the pre-built Övervakt image on Docker Hub as [bbqsrc/overvakt](https://hub.docker.com/r/bbqsrc/overvakt/).

> Pre-built Docker version may not be the latest version of Övervakt available.

First, pull the `bbqsrc/overvakt` image:

```bash
docker pull bbqsrc/overvakt:v1.24.3
```

Then, seed it a configuration file and run it (replace `/path/to/your/overvakt/overvakt.toml` with the path to your configuration file):

```bash
docker run -p 8080:8080 -v /path/to/your/overvakt/overvakt.toml:/etc/overvakt.cfg bbqsrc/overvakt:v1.24.3
```

In the configuration file, ensure that:

* `server.inet` is set to `0.0.0.0:8080` (this lets Övervakt be reached from outside the container)
* `assets.path` is set to `./res/assets/` (this refers to an internal path in the container, as the assets are contained there)

Övervakt will be reachable from `http://localhost:8080`.
-->

### Configuration

Use the sample [overvakt.toml](https://github.com/bbqsrc/overvakt/blob/main/overvakt.toml) configuration file and adjust it to your own environment.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production
* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:8080`) — Host and TCP port the Övervakt public status page should listen on
* `workers` (type: _integer_, allowed: any number, default: `4`) — Number of workers for the Övervakt public status page to run on
* `manager_token` (type: _string_, allowed: secret token, default: no default) — Manager secret token (ie. secret password)
* `reporter_token` (type: _string_, allowed: secret token, default: no default) — Reporter secret token (ie. secret password)

**[assets]**

* `path` (type: _string_, allowed: unix path, default: `./res/assets/`) — Path to Övervakt assets directory

**[branding]**

* `page_title` (type: _string_, allowed: any string, default: `Status Page`) — Status page title
* `page_url` (type: _string_, allowed: URL, no default) — Status page URL
* `company_name` (type: _string_, allowed: any string, no default) — Company name (ie. your company)
* `icon_color` (type: _string_, allowed: hexadecimal color code, no default) — Icon color (ie. your icon background color)
* `icon_url` (type: _string_, allowed: URL, no default) — Icon URL, the icon should be your squared logo, used as status page favicon (PNG format recommended)
* `logo_color` (type: _string_, allowed: hexadecimal color code, no default) — Logo color (ie. your logo primary color)
* `logo_url` (type: _string_, allowed: URL, no default) — Logo URL, the logo should be your full-width logo, used as status page header logo (SVG format recommended)
* `website_url` (type: _string_, allowed: URL, no default) — Website URL to be used in status page header
* `support_url` (type: _string_, allowed: URL, no default) — Support URL to be used in status page header (ie. where users can contact you if something is wrong)
* `custom_html` (type: _string_, allowed: HTML, default: empty) — Custom HTML to include in status page `head` (optional)

**[metrics]**

* `poll_interval` (type: _integer_, allowed: seconds, default: `120`) — Interval for which to probe nodes in `poll` mode
* `poll_retry` (type: _integer_, allowed: seconds, default: `2`) — Interval after which to try probe for a second time nodes in `poll` mode (only when the first check fails)
* `poll_http_status_healthy_above` (type: _integer_, allowed: HTTP status code, default: `200`) — HTTP status above which `poll` checks to HTTP replicas reports as `healthy`
* `poll_http_status_healthy_below` (type: _integer_, allowed: HTTP status code, default: `400`) — HTTP status under which `poll` checks to HTTP replicas reports as `healthy`
* `poll_delay_dead` (type: _integer_, allowed: seconds, default: `10`) — Delay after which a node in `poll` mode is to be considered `dead` (ie. check response delay)
* `poll_delay_sick` (type: _integer_, allowed: seconds, default: `5`) — Delay after which a node in `poll` mode is to be considered `sick` (ie. check response delay)
* `poll_parallelism` (type: _integer_, allowed: any number, default: `4`) — Maximum number of poll threads to be ran simultaneously (in case you are monitoring a lot of nodes and/or slow-replying nodes, increasing parallelism will help)
* `push_delay_dead` (type: _integer_, allowed: seconds, default: `20`) — Delay after which a node in `push` mode is to be considered `dead` (ie. time after which the node did not report)
* `push_system_cpu_sick_above` (type: _float_, allowed: system CPU loads, default: `0.90`) — System load indice for CPU above which to consider a node in `push` mode `sick` (ie. unix system load)
* `push_system_ram_sick_above` (type: _float_, allowed: system RAM loads, default: `0.90`) — System load indice for RAM above which to consider a node in `push` mode `sick` (ie. percent RAM used)
* `script_interval` (type: _integer_, allowed: seconds, default: `300`) — Interval for which to probe nodes in `script` mode
* `script_parallelism` (type: _integer_, allowed: any number, default: `2`) — Maximum number of script executor threads to be ran simultaneously (in case you are running a lot of scripts and/or long-running scripts, increasing parallelism will help)
* `local_delay_dead` (type: _integer_, allowed: seconds, default: `40`) — Delay after which a node in `local` mode is to be considered `dead` (ie. time after which the node did not report)

**[plugins]**

**[plugins.rabbitmq]**

* `api_url` (type: _string_, allowed: URL, no default) — RabbitMQ API URL (ie. `http://127.0.0.1:15672`)
* `auth_username` (type: _string_, allowed: username, no default) — RabbitMQ API authentication username
* `auth_password` (type: _string_, allowed: password, no default) — RabbitMQ API authentication password
* `virtualhost` (type: _string_, allowed: virtual host, no default) — RabbitMQ virtual host hosting the queues to be monitored
* `queue_ready_healthy_below` (type: _integer_, allowed: any number, no default) — Maximum number of payloads in RabbitMQ queue with status `ready` to consider node `healthy`.
* `queue_nack_healthy_below` (type: _integer_, allowed: any number, no default) — Maximum number of payloads in RabbitMQ queue with status `nack` to consider node `healthy`.
* `queue_ready_dead_above` (type: _integer_, allowed: any number, no default) — Threshold on the number of payloads in RabbitMQ queue with status `ready` above which node should be considered `dead` (stalled queue)
* `queue_nack_dead_above` (type: _integer_, allowed: any number, no default) — Threshold on the number of payloads in RabbitMQ queue with status `nack` above which node should be considered `dead` (stalled queue)
* `queue_loaded_retry_delay` (type: _integer_, allowed: milliseconds, no default) — Re-check queue if it reports as loaded after delay; this avoids false-positives if your systems usually take a bit of time to process pending queue payloads (if any)

**[notify]**

* `startup_notification` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to send startup notification or not (stating that systems are `healthy`)
* `reminder_interval` (type: _integer_, allowed: seconds, no default) — Interval at which downtime reminder notifications should be sent (if any)
* `reminder_backoff_function` (type _string_, allowed: `none`, `linear`, `square`, `cubic`, default: `none`) — If enabled, the downtime reminder interval will get larger as reminders are sent. The value will be `reminder_interval × pow(N, x)` with `N` being the number of reminders sent since the service went down, and `x` being the specified growth factor.
* `reminder_backoff_limit` (type: _integer_, allowed: any number, default: `3`) — Maximum value for the downtime reminder backoff counter (if a backoff function is enabled).

**[notify.email]**

* `to` (type: _string_, allowed: email address, no default) — Email address to which to send emails
* `from` (type: _string_, allowed: email address, no default) — Email address from which to send emails
* `smtp_host` (type: _string_, allowed: hostname, IPv4, IPv6, default: `localhost`) — SMTP host to connect to
* `smtp_port` (type: _integer_, allowed: TCP port, default: `587`) — SMTP TCP port to connect to
* `smtp_username` (type: _string_, allowed: any string, no default) — SMTP username to use for authentication (if any)
* `smtp_password` (type: _string_, allowed: any string, no default) — SMTP password to use for authentication (if any)
* `smtp_encrypt` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to encrypt SMTP connection with `STARTTLS` or not
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send emails only for downtime reminders or everytime

**[notify.twilio]**

* `to` (type: _array[string]_, allowed: phone numbers, no default) — List of phone numbers to which to send text messages
* `service_sid` (type: _string_, allowed: any string, no default) — Twilio service identifier (ie. `Service Sid`)
* `account_sid` (type: _string_, allowed: any string, no default) — Twilio account identifier (ie. `Account Sid`)
* `auth_token` (type: _string_, allowed: any string, no default) — Twilio authentication token (ie. `Auth Token`)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send text messages only for downtime reminders or everytime

**[notify.slack]**

* `hook_url` (type: _string_, allowed: URL, no default) — Slack hook URL (ie. `https://hooks.slack.com/[..]`)
* `mention_channel` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to mention channel when sending Slack messages (using _@channel_, which is handy to receive a high-priority notification)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send Slack messages only for downtime reminders or everytime

**[notify.zulip]**

* `bot_email` (type: _string_, allowed: any string, no default) — The bot mail address as given by the Zulip interface
* `bot_api_key` (type: _string_, allowed: any string, no default) — The bot API key as given by the Zulip interface
* `channel` (type: _string_, allowed: any string, no default) — The name of the channel to send notifications to
* `api_url` (type: _string_, allowed: URL, no default) — The API endpoint url (eg. `https://domain.zulipchat.com/api/v1/`)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send messages only for downtime reminders or everytime

**[notify.telegram]**

* `bot_token` (type: _string_, allowed: any strings, no default) — [Telegram bot token](https://core.telegram.org/bots/api#authorizing-your-bot)
* `chat_id` (type: _string_, allowed: any strings, no default) — Chat identifier where you want Övervakt to send messages. Can be group chat identifier (eg. `"@foo"`) or user chat identifier (eg. `"123456789"`)

**[notify.pushover]**

* `app_token` (type: _string_, allowed: any string, no default) — Pushover application token (you need to create a dedicated Pushover application to get one)
* `user_keys` (type: _array[string]_, allowed: any strings, no default) — List of Pushover user keys (ie. the keys of your Pushover target users for notifications)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send Pushover notifications only for downtime reminders or everytime

**[notify.gotify]**

* `app_url` (type: _string_, allowed: URL, no default) - Gotify endpoint without trailing slash (eg. `https://push.gotify.net`)
* `app_token` (type: _string_, allowed: any string, no default) — Gotify application token
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send Gotify notifications only for downtime reminders or everytime

**[notify.xmpp]**

**Notice: the XMPP notifier requires `libstrophe` (`libstrophe-dev` package on Debian) to be available when compiling Övervakt, with the feature `notifier-xmpp` enabled upon Cargo build.**

* `to` (type: _string_, allowed: Jabber ID, no default) — Jabber ID (JID) to which to send messages
* `from` (type: _string_, allowed: Jabber ID, no default) — Jabber ID (JID) from which to send messages
* `xmpp_password` (type: _string_, allowed: any string, no default) — XMPP account password to use for authentication
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send messages only for downtime reminders or everytime

**[notify.matrix]**

* `homeserver_url` (type: _string_, allowed: URL, no default) — Matrix server where the account has been created (eg. `https://matrix.org`)
* `access_token` (type: _string_, allowed: any string, no default) — Matrix access token from a previously created session (eg. Element Web access token)
* `room_id` (type: _string_, allowed: any string, no default) — Matrix room ID to which to send messages (eg. `!abc123:matrix.org`)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send messages only for downtime reminders or everytime

**[notify.webex]**

* `endpoint_url` (type: _string_, allowed: URL, no default) — Webex endpoint URL (eg. `https://webexapis.com/v1/messages`)
* `token` (type: _string_, allowed: any string, no default) - Webex access token
* `room_id` (type: _string_, allowed: any string, no default) - Webex room ID to which to send messages (eg. `Y2lzY29zcGFyazovL3VzL1JPT00vMmJmOD`)
* `reminders_only` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to send messages only for downtime reminders or everytime

**[notify.webhook]**

* `hook_url` (type: _string_, allowed: URL, no default) — Web Hook URL (eg. `https://domain.com/webhooks/[..]`)

**[probe]**

**[[probe.service]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service (not visible on the status page)
* `label` (type: _string_, allowed: any string, no default) — Name of the probed service (visible on the status page)

**[[probe.service.node]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service node (not visible on the status page)
* `label` (type: _string_, allowed: any string, no default) — Name of the probed service node (visible on the status page)
* `mode` (type: _string_, allowed: `poll`, `push`, `script`, `local`, no default) — Probe mode for this node (ie. `poll` is direct HTTP, TCP or ICMP poll to the URLs set in `replicas`, while `push` is for Övervakt Reporter nodes, `script` is used to execute a shell script and `local` is for Övervakt Local nodes)
* `replicas` (type: _array[string]_, allowed: TCP, ICMP or HTTP URLs, default: empty) — Node replica URLs to be probed (only used if `mode` is `poll`)
* `scripts` (type: _array[string]_, allowed: shell scripts as source code, default: empty) — Shell scripts to be executed on the system as a Övervakt sub-process; they are handy to build custom probes (only used if `mode` is `script`)
* `http_headers` (type: _map[string, string]_, allowed: any valid header name and value, default: empty) — HTTP headers to add to HTTP requests (eg. `http_headers = { "Authorization" = "Bearer xxxx" }`)
* `http_method` (type _string_, allowed: `GET`, `HEAD`, `POST`, `PUT`, `PATCH`, no default) — HTTP method to use when polling the endpoint (omitting this will default to using `HEAD` or `GET` depending on the `http_body_healthy_match` configuration value)
* `http_body` (type _string_, allowed: any string, no default) — Body to send in the HTTP request when polling an endpoint (this only works if `http_method` is set to `POST`, `PUT` or `PATCH`)
* `http_body_healthy_match` (type: _string_, allowed: regular expressions, no default) — HTTP response body for which to report node replica as `healthy` (if the body does not match, the replica will be reported as `dead`, even if the status code check passes; the check uses a `GET` rather than the usual `HEAD` if this option is set)
* `rabbitmq_queue` (type: _string_, allowed: RabbitMQ queue names, no default) — RabbitMQ queue associated to node, which to check against for pending payloads via RabbitMQ API (this helps monitor unacked payloads accumulating in the queue)
* `rabbitmq_queue_nack_healthy_below` (type: _integer_, allowed: any number, no default) — Maximum number of payloads in RabbitMQ queue associated to node, with status `nack` to consider node `healthy` (this overrides the global `plugins.rabbitmq.queue_nack_healthy_below`)
* `rabbitmq_queue_nack_dead_above` (type: _integer_, allowed: any number, no default) — Threshold on the number of payloads in RabbitMQ queue associated to node, with status `nack` above which node should be considered `dead` (stalled queue, this overrides the global `plugins.rabbitmq.queue_nack_dead_above`)

### Run Övervakt

```bash
./overvakt -c /path/to/overvakt.toml
```

## Usage recommendations

**Consider the following recommendations when using Övervakt:**

* **Övervakt should be hosted on a safe, separate server.** This server should run on a different physical machine and network than your monitored infrastructure servers.
* **Make sure to whitelist the Övervakt server public IP (both IPv4 and IPv6)** on your monitored HTTP services; this applies if you use a bot protection service that challenges bot IPs, eg. Distil Networks or Cloudflare. Övervakt will see the HTTP service as down if a bot challenge is raised.

## What status variants look like?

Övervakt has 3 status variants, either `healthy` (no issue ongoing), `sick` (services under high load) or `dead` (outage):

### Healthy status variant

![Status Healthy](https://bbqsrc.github.io/overvakt/images/status-healthy.png)

### Sick status variant

![Status Sick](https://bbqsrc.github.io/overvakt/images/status-sick.png)

### Dead status variant

![Status Dead](https://bbqsrc.github.io/overvakt/images/status-dead.png)

## What do announcements look like?

Announcements can be published to let your users know about any planned maintenance, as well as your progress on resolving a downtime:

![Announcement](https://bbqsrc.github.io/overvakt/images/announcement.png)

## What do alerts look like?

When a monitored backend or app goes down in your infrastructure, Övervakt can let you know by Slack, Twilio SMS, Email and XMPP:

![Övervakt alert in Slack](https://bbqsrc.github.io/overvakt/images/alert-slack.png)

You can also get nice realtime `down` and `up` alerts on your eg. iPhone and Apple Watch:

<p align="center">
  <img src="https://bbqsrc.github.io/overvakt/images/alert-slack-iphone.png" width="210" alt="Övervakt down alert on iPhone (Slack)" />
  <img src="https://bbqsrc.github.io/overvakt/images/alert-slack-watch.jpg" width="398" alt="Övervakt up alert on Apple Watch (Slack)" />
  <img src="https://bbqsrc.github.io/overvakt/images/alert-twilio-iphone.png" width="210" alt="Övervakt alerts on iPhone (Twilio SMS)" />
<p>

## What do Webhook payloads look like?

If you are using the Webhook notifier in Övervakt, you will receive a JSON-formatted payload with alert details upon any status change; plus reminders if `notify.reminder_interval` is configured.

**Here is an example of a Webhook payload:**

```json
{
  "type": "changed",
  "status": "dead",
  "time": "08:58:28 UTC+0200",

  "replicas": [
    "web:core:tcp://edge-3.pool.net.crisp.chat:80"
  ],

  "page": {
    "title": "Crisp Status",
    "url": "https://status.crisp.chat/"
  }
}
```

Webhook notifications can be tested with eg. [Webhook.site](https://webhook.site/), before you integrate them to your custom endpoint.

_You can use those Webhook payloads to create custom notifiers to anywhere. For instance, if you are using Microsoft Teams but not Slack, you may write a tiny PHP script that receives Webhooks from Övervakt and forwards a notification to Microsoft Teams. This can be handy; while Övervakt only implements convenience notifiers for some selected channels, the Webhook notifier allows you to extend beyond that._

## How can I create script probes?

Övervakt lets you create custom probes written as shell scripts, passed in the Övervakt configuration as a list of scripts to be executed for a given node.

Those scripts can be used by advanced Övervakt users when their monitoring use case requires scripting, ie. when `push` and `poll` probes are not enough.

The replica health should be returned by the script shell as return codes, where:

* **`rc=0`**: `healthy`
* **`rc=1`**: `sick`
* **`rc=2` and higher**: `dead`

As scripts are usually multi-line, script contents can be passed as a literal string, enclosed between `'''`.

As an example, the following script configuration always return as `sick`:

```
scripts = [
  '''
  # Do some work...
  exit 1
  '''
]
```

_Note that scripts are executed in a system shell ran by a Övervakt-owned sub-process. Make sure that Övervakt runs on an unix user with limited privileges. Running Övervakt as root would let any configured script perform root-level actions on the machine, which is not recommended._

## Troubleshoot Issues

### ICMP replicas always report as `dead`

On Linux systems, non-priviledge users cannot create raw sockets, which Övervakt ICMP probing system requires. It means that, by default, all ICMP probe attempts will fail silently, as if the host being probed was always down.

This can easily be fixed by allowing Övervakt to create raw sockets:

```bash
setcap 'cap_net_raw+ep' /bin/overvakt
```

_Note that HTTP and TCP probes do not require those raw socket capabilities._
