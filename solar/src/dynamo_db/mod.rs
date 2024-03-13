
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

pub async fn save_solar_system(name: &str) {
    println!("Saving solar system {name} to DynamoDB");

    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("1".to_string()));
    item.insert("name".to_string(), AttributeValue::S(name.to_string()));
    item.insert("type".to_string(), AttributeValue::S("solar_system".to_string()));

    let input = aws_sdk_dynamodb::types::PutItemInput {
        item,
        table_name: "solar_systems".to_string(),
        ..Default::default()
    };

    let result = client.put_item(input).await.unwrap();
    println!("Result: {:?}", result);
    return;
}
