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
// Copyright: 2021, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use http::header::HeaderName;
use poem::{
    error::InternalServerError,
    handler,
    web::{Data, Html, Path, StaticFileRequest},
    FromRequest, IntoResponse, Request, Response,
};
use tera::Tera;

use super::announcements::STORE as ANNOUNCEMENTS_STORE;
use super::context::{IndexContext, INDEX_CONFIG, INDEX_ENVIRONMENT};
use crate::prober::manager::STORE as PROBER_STORE;
use crate::APP_CONF;

#[handler]
pub(crate) async fn index(tera: Data<&Tera>) -> poem::Result<Html<String>> {
    // Notice acquire lock in a block to release it ASAP (ie. before template renders)
    let context = IndexContext {
        states: &PROBER_STORE.read().states,
        announcements: &ANNOUNCEMENTS_STORE.read().announcements,
        environment: &*INDEX_ENVIRONMENT,
        config: &*INDEX_CONFIG,
    };

    let render = tera.render(
        "index.tera",
        &tera::Context::from_serialize(context).unwrap(),
    );

    match render {
        Ok(s) => Ok(Html(s)),
        Err(e) => Err(InternalServerError(e)),
    }
}

#[handler]
pub(crate) async fn status_text() -> &'static str {
    &PROBER_STORE.read().states.status.as_str()
}

#[handler]
pub(crate) async fn badge(Path(kind): Path<String>) -> Response {
    // Notice acquire lock in a block to release it ASAP (ie. before OS access to file)
    let status = { &PROBER_STORE.read().states.status.as_str() };

    let req = StaticFileRequest::from_request_without_body(&Request::builder().finish())
        .await
        .unwrap();

    let badge_path = APP_CONF
        .assets
        .path
        .join("images")
        .join("badges")
        .join(format!("{}-{}-default.svg", kind, status));

    let resp = req.create_response(&badge_path, false).unwrap();
    let mut resp = resp.into_response();

    let headers = resp.headers_mut();
    headers.remove(&HeaderName::from_lowercase(b"last-modified").unwrap());

    resp
}
