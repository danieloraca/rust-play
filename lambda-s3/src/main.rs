use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
struct Request {
    bucket: String,
    name: String,
    version: u16,
}

#[derive(Serialize)]
struct Response {
    content: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Value, Error> {
    let name: String = event.payload.name;
    let version: u16 = event.payload.version;
    let bucket: String = event.payload.bucket;

    let key: String = format!("src/{}/{}/{}.json", &name, version, name);
    let s3_client: S3Client = S3Client::new(Region::EuWest1);

    let output = s3_client
        .get_object(rusoto_s3::GetObjectRequest {
            bucket,
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

    Ok(json_value)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
