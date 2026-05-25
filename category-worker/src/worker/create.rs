use serde_json::Value;

pub fn handler(payload: Value) {
    eprintln!("create event: {}", payload["orderid"]);
}

