use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    let payload_data = payload["body"].clone();
    // let payload_data = if let Some(body_value) = &payload.get("body") {
    //     if let Some(body_str) = body_value.as_str() {
    //         body_str
    //     } else {
    //         "No body found1"
    //     }
    // } else {
    //     match payload["body"].clone() {
    //         Value::String(_body_str) => "yay",
    //         _ => "No body found2",
    //     }
    // };

    let result: u64 = heavy_computation(1000000);

    let json_value = json!({
        "message": "Howdy!",
        "payload": payload_data,
        "result": result
    });

    Ok(json_value)
}

fn heavy_computation(limit: u64) -> u64 {
    let mut sum = 0;

    for i in 1..=limit {
        let square = i * i;

        sum += square;
    }

    sum
}
