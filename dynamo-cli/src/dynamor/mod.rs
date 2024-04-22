extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
extern crate serde_dynamodb;
use crate::transformers::integrations;
use crate::transformers::mapped_fields;
use crate::transformers::modules;
use std::env;

use crate::types::{Integration, MappedField, Module};

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};
use std::collections::HashMap;

pub async fn get_integration(integration_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    let integrations_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match integrations_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| integrations::process_integrations_item(item))
                .collect::<Vec<Integration>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    let serialized = serde_json::to_string_pretty(&items).unwrap();

    Ok(serialized)
}

pub async fn get_mapped_field(integration_id: &str, mapped_field_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();
    let sk = format!("{}#{}", integration_id, mapped_field_id);
    // I#01HV177W1JAS01D5J3EZDSKCC0
    // I#01HV177W1JAS01D5J3EZDSKCC0#F#01HV1780HDZNHP669VFJ46KBTH

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(sk)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| mapped_fields::process_mapped_field_item(item))
                .collect::<Vec<MappedField>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    let serialized = serde_json::to_string_pretty(&items).unwrap();
    Ok(serialized)
}

pub async fn get_all_mapped_fields_for_integration(integration_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();
    let sk = format!("{}#F", integration_id);

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(sk)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and begins_with(SK, :sk)")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| mapped_fields::process_mapped_field_item(item))
                .collect::<Vec<MappedField>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    let serialized = serde_json::to_string_pretty(&items).unwrap();
    Ok(serialized)
}

pub async fn get_module(integration_id: &str, module_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();
    let sk = format!("{}#{}", integration_id, module_id);

    // I#01HV177W1JAS01D5J3EZDSKCC0
    // M#01HV1780JP8JEFCTGZ63VZ6TDJ
    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(sk)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| modules::process_module_item(item))
                .collect::<Vec<Module>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    let serialized = serde_json::to_string_pretty(&items).unwrap();
    Ok(serialized)
}

pub async fn get_all_modules_for_integration(integration_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();
    let sk = format!("{}#M", integration_id);

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(integration_id)),
            ..Default::default()
        },
    );

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(sk)),
            ..Default::default()
        },
    );

    let items: Vec<Module> = match client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and begins_with(SK, :sk)")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await
    {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| modules::process_module_item(item))
                .collect(),
            None => {
                eprintln!("No items found in the query result.");
                Vec::new()
            }
        },
        Err(err) => {
            eprintln!("Error querying integrations: {}", err);
            Vec::new()
        }
    };

    let serialized = serde_json::to_string_pretty(&items).unwrap();
    Ok(serialized)
}

fn setup_aws_client() -> DynamoDbClient {
    let aws_region = env::var("AWS_REGION").unwrap_or_else(|_| "eu-west-1".to_string());
    let region = match aws_region.as_str() {
        "eu-west-1" => Region::EuWest1,
        "us-east-1" => Region::UsEast1,
        _ => {
            eprintln!("Invalid region: {}", aws_region);
            std::process::exit(1);
        }
    };

    DynamoDbClient::new(region)
}
