mod http_layer;

use app::{handler, Error, Event};
use http_layer::HttpParserLayer;
use hyper::{
    body::Buf, header, service::make_service_fn, Body, Request, Response, Server, StatusCode,
};
use serde_json::Value;
use std::result;
use std::{convert::Infallible, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

type Result<T> = result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:1337".parse().unwrap();

    let make_service = make_service_fn(|_conn| async {
        let service = ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(60 * 3)))
            .layer(HttpParserLayer::new())
            .service(tower::service_fn(handler));
        Ok::<_, Infallible>(service)
    });

    Server::bind(&addr).serve(make_service).await?;

    Ok(())
}
