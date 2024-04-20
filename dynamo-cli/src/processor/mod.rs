use crate::dynamo::ListItemsResult;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamodb;
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    OwnId: String,
    SecCon: SecondaryConnection,
    PriCon: PrimaryConnection,
    SK: String,
    PriAuth: String,
    UpAt: String,
    PK: String,
    IStatus: IntegrationStatus,
    CrAt: String,
    SecAuth: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecondaryConnection {
    connectionName: String,
    accountId: String,
    connectionType: String,
    api_domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrimaryConnection {
    connectionType: String,
    connectionName: String,
    accountId: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IntegrationStatus {
    setupComplete: SetupComplete,
    auth: AuthStatus,
}

#[derive(Debug, Serialize, Deserialize)]
struct SetupComplete {
    primary: bool,
    secondary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthStatus {
    secondary: AuthDetail,
    primary: AuthDetail,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthDetail {
    code: String,
}

pub fn process_integration(list_items_result: &Vec<HashMap<String, AttributeValue>>) -> &str {
    // let json_values: Vec<Value> = list_items_result
    //     .iter()
    //     .map(|item| {
    //         let mut json_item = HashMap::new();
    //         item.iter().for_each(|(key, value)| {
    //             let key = key.to_string();
    //             let value = "value".to_string();
    //             json_item.insert(key, value);
    //         });
    //         let json_item = from_str(&serde_json::to_string(&json_item).unwrap()).unwrap();
    //         json_item
    //     })
    //     .collect();

    // println!("JSON: {:?}", json_values);

    let j: &str = r#"{
            "name": "John Doe",
            "age": 30,
            "city": "New York"
        }"#;
    let ex: Value = serde_json::from_str(j).expect("msg");
    let name: &str = ex["name"].as_str().unwrap();
    println!("{:?}", name);

    list_items_result.iter().for_each(|item| {
        println!("ITEM IS {:?}", item);

        let atv: AttributeValue = item.get("OwnId").unwrap().clone();
        println!("{:?}", atv);
        panic!("ffs");
    });
    println!("{:?}", list_items_result);
    return "ffs";
}
