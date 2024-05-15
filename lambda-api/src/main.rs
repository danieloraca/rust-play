use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    // let a = payload.clone();

    // let payload_data = if let Some(body_value) = &payload.get("body") {
    //     if let Some(body_str) = body_value.as_str() {
    //         body_str
    //     } else {
    //         "No body found1"
    //     }
    // } else {
    //     payload.to_string().as_str()
    // };

    let result: u64 = heavy_computation(1_000_000).await;

    // tokio_thread1().await;
    let response_thread_2 = tokio_thread2().await;

    let json_value = json!({
        "message": "Howdy!",
        "payload": payload,
        "result": result,
        "response_thread_2": response_thread_2,
    });

    Ok(json_value)
}

async fn ticker() {
    for i in 0..10 {
        println!("TICKER: {}", i);
        tokio::task::yield_now().await;
    }
}

async fn tocker() {
    for i in 0..10 {
        println!("TOCKER: {}", i);
        tokio::task::yield_now().await;
    }
}

async fn heavy_computation(limit: u64) -> u64 {
    let mut sum = 0;

    for i in 1..=limit {
        let square = i * i;

        sum += square;
    }

    sum
}

async fn tokio_thread1() -> () {
    println!("Starting tokio thread 1");
    let _ = tokio::join!(tokio::spawn(ticker()), tokio::spawn(tocker()));
}

async fn tokio_thread2() -> Vec<String> {
    // println!("Starting tokio thread 2");
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(10);
    let handle_tokio = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        let mut n: u32 = 0;
        while n < 100 {
            // std::thread::sleep(std::time::Duration::from_nanos(1));
            let my_tx = tx.clone();
            handle_tokio.spawn(async move {
                my_tx.send(n).await.unwrap();
            });
            n += 1;
        }
    });

    let mut response: Vec<String> = vec![];

    while let Some(n) = rx.recv().await {
        // println!("Received: {} from the system thread", n);
        response.push(format!("Received: {} from the system thread", n));
    }

    response
}
