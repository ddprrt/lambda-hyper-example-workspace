use serde_json::{json, Value};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Event(pub Value);

pub async fn handler(event: Event) -> Result<Value, Error> {
    let val = event.0;
    let first_name = val["firstName"].as_str().unwrap_or("world");
    Ok(json!({ "message": format!("Hello, {}", first_name) }))
}
