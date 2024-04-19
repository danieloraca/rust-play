use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ListItemsResult {
    Array(Vec<HashMap<String, AttributeValue>>),
}

pub async fn get_integration(integration_id: &str) -> Result<ListItemsResult, Error> {
    println!("Querying for integration Id: {}", integration_id);

    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);
    let result = client
        .query()
        .table_name("StageIntegrations")
        .key_condition_expression("#pk = :pk adn #sk = :sk")
        .expression_attribute_names("#pk", "PK")
        .expression_attribute_names("#sk", "SK")
        .expression_attribute_values(":pk", AttributeValue::S(integration_id.to_string()))
        .send()
        .await?;

    if let Some(items) = result.items {
        println!("Found {} items", items.len());
        return Ok(ListItemsResult::Array(items));
    }

    println!("No items found");
    Ok(ListItemsResult::Array(vec![]))
}
