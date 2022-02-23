mod http_layer;

use app::{handler, Error};
use http_layer::HttpParserLayer;
use hyper::{service::make_service_fn, Server};

use std::net::SocketAddr;
use std::{convert::Infallible, time::Duration};
use std::{env, result};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

type Result<T> = result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let addr = "127.0.0.1".parse().unwrap();

    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom handler port is not a number"),
        Err(_) => 1337,
    };

    let addr = SocketAddr::new(addr, port);

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
