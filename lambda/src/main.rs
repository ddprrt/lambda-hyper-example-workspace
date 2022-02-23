mod lambda_event_layer;

use std::time::Duration;

use lambda_event_layer::LambdaEventLayer;
use lambda_runtime::service_fn;

use app::{handler, Error};

use tower::{timeout::TimeoutLayer, ServiceBuilder};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    let func = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(3 * 60)))
        .layer(LambdaEventLayer::new())
        .service(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
