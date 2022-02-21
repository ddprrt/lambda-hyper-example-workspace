use lambda_runtime::{service_fn, LambdaEvent};

use app::{handler, Error, Event};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (val, _) = event.into_parts();
    handler(Event(val)).await
}
