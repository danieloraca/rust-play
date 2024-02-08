use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Request {
    id: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    result: Vec<SolarSystem>,
}

#[derive(Serialize)]
pub struct SolarSystem {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub system_type: String,
}

impl SolarSystem {
    pub fn new(id: String, name: String, system_type: String) -> Self {
        SolarSystem {
            id,
            name,
            system_type,
        }
    }
}

fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}

impl From<&HashMap<String, AttributeValue>> for SolarSystem {
    fn from(item: &HashMap<String, AttributeValue>) -> Self {
        let id = as_string(item.get("id"), &"".to_string());
        let name = as_string(item.get("name"), &"".to_string());
        let system_type = as_string(item.get("type"), &"".to_string());

        SolarSystem::new(id, name, system_type)
    }
}

pub async fn query_by_id(
    client: &Client,
    table_name: &str,
    id: &str,
) -> Result<Vec<SolarSystem>, Error> {
    let result = client
        .query()
        .table_name(table_name)
        .key_condition_expression("id = :id")
        .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
        .send()
        .await?;

    if let Some(items) = result.items {
        let solar_systems = items
            .iter()
            .map(|v| v.into())
            .collect();
        Ok(solar_systems)
    } else {
        Ok(vec![])
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let id = event.payload.id;
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    let res = match query_by_id(&client, "DynamoDan", &id).await {
        Ok(solar_systems) => axum::Json(solar_systems),
        Err(_e) => axum::Json(Vec::new()),
    };

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        result: res.0
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
