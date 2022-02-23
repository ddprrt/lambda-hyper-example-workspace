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

async fn _handle(req: Request<Body>) -> Result<Response<Body>> {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await?;
    // Decode as JSON...
    let data: Value = serde_json::from_reader(whole_body.reader())?;

    let json = handler(Event(data)).await?.to_string();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}
