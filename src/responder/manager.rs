// Vigil
//
// Microservices Status Page
// Copyright: 2021, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use poem::{
    endpoint::StaticFilesEndpoint,
    get,
    listener::TcpListener,
    middleware::{NormalizePath, TrailingSlash},
    EndpointExt, Route, Server,
};
use tera::Tera;

use super::routes;
use crate::APP_CONF;

pub async fn run() -> std::io::Result<()> {
    let templates: String = APP_CONF
        .assets
        .path
        .canonicalize()
        .unwrap()
        .join("templates")
        .join("*")
        .to_str()
        .unwrap()
        .into();

    let tera = Tera::new(&templates).unwrap();

    let app = Route::new()
        .at("/", get(routes::index))
        .at("/status/text", get(routes::status_text))
        .at("/badge/:kind", get(routes::badge))
        .nest("/assets", StaticFilesEndpoint::new(&APP_CONF.assets.path))
        .data(tera.clone())
        .with(NormalizePath::new(TrailingSlash::Trim));

    Server::new(TcpListener::bind(APP_CONF.server.inet))
        .run(app)
        .await?;

    Ok(())
}
