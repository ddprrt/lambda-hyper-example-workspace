use std::fmt::{Debug, Display};

use anyhow::Result;
use app::Event;
use futures::future::BoxFuture;
use hyper::body::Buf;
use hyper::service::Service;
use hyper::{header, Body, Request, Response, StatusCode};
use serde_json::Value;

pub struct HttpParserLayer;

impl HttpParserLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> tower::Layer<S> for HttpParserLayer {
    type Service = HttpParser<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpParser::new(inner)
    }
}

#[derive(Clone, Copy)]
pub struct HttpParser<S> {
    inner: S,
}

impl<S> HttpParser<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request<Body>> for HttpParser<S>
where
    S: tower::Service<Event> + Clone + Send + 'static,
    S::Future: Send,
    S::Response: Debug + Display,
{
    type Response = Response<Body>;
    type Error = anyhow::Error;
    type Future = BoxFuture<'static, Result<Response<Body>>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut this = self.inner.clone();
        Box::pin(async move {
            let data = prepare(req).await?;

            let result = this.call(Event(data)).await;

            match result {
                Ok(result) => {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(result.to_string()))?;
                    Ok(response)
                }
                Err(_) => {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::empty())?;
                    Ok(response)
                }
            }
        })
    }
}

async fn prepare(req: Request<Body>) -> Result<Value> {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await?;
    // Decode as JSON...
    let data: Value = serde_json::from_reader(whole_body.reader())?;

    Ok(data)
}
