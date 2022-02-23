use app::Event;
use lambda_runtime::LambdaEvent;
use serde_json::Value;

pub struct LambdaEventLayer;

impl LambdaEventLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> tower::Layer<S> for LambdaEventLayer {
    type Service = LambdaEventParser<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LambdaEventParser::new(inner)
    }
}

#[derive(Copy, Clone)]
pub struct LambdaEventParser<S> {
    inner: S,
}

impl<S> LambdaEventParser<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> tower::Service<LambdaEvent<Value>> for LambdaEventParser<S>
where
    S: tower::Service<Event>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LambdaEvent<Value>) -> Self::Future {
        let (val, _) = req.into_parts();
        self.inner.call(Event(val))
    }
}
