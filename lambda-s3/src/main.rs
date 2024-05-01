use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{S3Client, S3};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct Response {
    content: String,
}

#[derive(Clone)]
struct CacheItem {
    value: Value,
    expiration_time: Instant,
}

static mut CACHE: Option<Arc<Mutex<HashMap<String, CacheItem>>>> = None;

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    let body_json: Value = serde_json::from_str(
        payload["body"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid body"))?,
    )
    .map_err(|e| -> Error { anyhow::anyhow!("Error parsing JSON: {}", e).into() })?;

    let bucket = match env::var("BUCKET") {
        Ok(val) => val,
        Err(_) => return Err(anyhow::anyhow!("No BUCKET environment variable found").into()),
    };
    let name = body_json["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No name found in payload"))?;
    let version = body_json["version"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No version found in payload"))?;

    tracing::info!("Payload processed: Name: {}, Version: {}", name, version);

    let key = format!("src/{}/{}/{}.json", name, version, name);

    // Check cache first
    let cache_result;
    unsafe {
        cache_result = CACHE.as_ref().and_then(|cache| {
            let cache = cache.lock().unwrap();
            cache.get(&key).cloned()
        });
    }
    if let Some(cache_item) = cache_result {
        // Check if cache item is expired
        if Instant::now() < cache_item.expiration_time {
            return Ok(cache_item.value);
        }
    }

    let s3_client: S3Client = S3Client::new(Region::EuWest1);
    tracing::info!("Requesting {} from {}", key, bucket);

    let output = s3_client
        .get_object(rusoto_s3::GetObjectRequest {
            bucket: bucket.to_string(),
            key: key.clone(),
            ..Default::default()
        })
        .await?;

    let Some(body): Option<ByteStream> = output.body else {
        return Err(anyhow::anyhow!("No body found in S3 response").into());
    };

    // Read the byte stream into memory
    let mut content = Vec::new();
    body.into_async_read().read_to_end(&mut content).await?;

    // Convert the content to a string
    let content: String = String::from_utf8(content)?;
    let json_value: Value = serde_json::from_str(content.as_str())?;

    // Store JSON data in cache with expiration time
    let expiration_time = Instant::now() + Duration::from_secs(300); // 5 minutes expiration time
    let cache_item = CacheItem {
        value: json_value.clone(),
        expiration_time,
    };

    unsafe {
        if CACHE.is_none() {
            CACHE = Some(Arc::new(Mutex::new(HashMap::new())));
        }
        let cache = CACHE.as_ref().unwrap().clone();
        let mut cache = cache.lock().unwrap();
        cache.insert(key, cache_item);
    }

    Ok(json_value)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await
}
