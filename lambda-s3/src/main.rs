use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{S3Client, S3};
use serde::Serialize;
use serde_json::Value;
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct Response {
    content: String,
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    let body_json: Value = serde_json::from_str(
        payload["body"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid body"))?,
    )
    .map_err(|e| -> Error { anyhow::anyhow!("Error parsing JSON: {}", e).into() })?;

    let bucket = body_json["bucket"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No bucket found in payload"))?;
    let name = body_json["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No name found in payload"))?;
    let version = body_json["version"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No version found in payload"))?;

    tracing::info!("Payload processed: Name: {}, Version: {}", name, version);

    let key = format!("src/{}/{}/{}.json", name, version, name);
    let s3_client: S3Client = S3Client::new(Region::EuWest1);
    tracing::info!("Requesting {} from {}", key, bucket);

    let output = s3_client
        .get_object(rusoto_s3::GetObjectRequest {
            bucket: bucket.to_string(),
            key,
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
    // tracing::info!("JSON Value: {:?}", json_value);
    Ok(json_value)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await
}
