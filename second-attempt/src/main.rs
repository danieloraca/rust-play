use tracing_subscriber::filter::{EnvFilter, LevelFilter};use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Deserialize)]
struct Request {
    id: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    my_id: Option<i32>
}

// async fn query_by_id(client: &Client, table_name: String) {
//     let result = client.query()
//         .table_name(table_name)
//         .key_condition_expression("id = :id")
//         .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
//         .send()
//         .await?;
//
//     if let Some(items) = result.items {
//        for item in items {
//            println!("Item: {:?}", item);
//        }
//     }
//     Ok(())
// }
//
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let id = event.payload.id;
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    let result = client.query()
        .table_name("DynamoDan")
        .key_condition_expression("id = :id")
        .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
        .send()
        .await?;

    // let result = client
    //     .scan()
    //     .table_name("DynamoDan")
    //     .limit(3)
    //     .send()
    //     .await?;
    
    // let result = client
    //     .query(QueryInput {
    //         table_name: "DynamoDan".to_string(),
    //         key_condition_expression: Some("#id = :id".to_string()),
    //         expression_attribute_names: Some(maplit::hashmap! {
    //             "#id".to_string() => "id".to_string(),
    //         }),
    //         expression_attribute_values: Some(maplit::hashmap! {
    //             ":id".to_string() => rusoto_dynamodb::AttributeValue {
    //                 s: Some(id_to_query.to_string()),
    //                 ..Default::default()
    //             },
    //         }),
    //         limit: Some(3),
    //         ..Default::default()
    //     })
    //     .await?;

    // if result.is_ok() {
    //     println!("got something!");
    // }

    if let Some(items) = result.items {
       for item in items {
           println!("Item: {:?}", item);
       }
    }
    //
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Id {}.", id),
        my_id: Some(112),
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
