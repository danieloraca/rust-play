use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ListItemsResult {
    Array(Vec<HashMap<String, AttributeValue>>),
}

pub async fn get_integration(integration_id: &str) -> Result<ListItemsResult, Error> {
    println!("Querying for integration Id: {}", integration_id);

    //I#01HV177W1JAS01D5J3EZDSKCC0

    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    println!("{}", shared_config.region().unwrap().as_ref());
    let result = client
        .query()
        .table_name("Stage-Integrations")
        .key_condition_expression("#pk = :pk and #sk = :sk")
        .expression_attribute_names("#pk", "PK")
        .expression_attribute_names("#sk", "SK")
        .expression_attribute_values(":pk", AttributeValue::S(integration_id.to_string()))
        .expression_attribute_values(":sk", AttributeValue::S(integration_id.to_string()))
        .send()
        .await?;

    println!("Finished query");

    if let Some(items) = result.items {
        println!("Found {} items", items.len());
        return Ok(ListItemsResult::Array(items));
    }

    println!("No items found");
    Ok(ListItemsResult::Array(vec![]))
}
