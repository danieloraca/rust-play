extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
extern crate serde_dynamodb;
use crate::transformers::integrations;
use crate::transformers::logs;
use crate::transformers::mapped_fields;
use crate::transformers::modules;
use crate::transformers::syncs;
use crate::types::Log;
use crate::types::Sync;
use crate::types::{Integration, MappedField, Module};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};
use serde::Serialize;
use std::collections::HashMap;
use std::env;

#[derive(Serialize)]
enum ProcessedItem {
    Integration(Integration),
    MappedField(MappedField),
    Module(Module),
}

pub async fn get_integration_by_id(integration_id: &str) -> Result<Option<Vec<Integration>>, ()> {
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
                .map(|item| integrations::process_integration_item(item))
                .collect::<Vec<Integration>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(items))
    }
}

pub async fn get_integration(integration_id: &str) -> Result<String, ()> {
    let integration = get_integration_by_id(integration_id).await.unwrap();

    let serialized = serde_json::to_string_pretty(&integration).unwrap();

    Ok(serialized)
}

pub async fn get_all_integrations_for_owner(owner_id: &str) -> Result<String, ()> {
    // 02f6b978-f97f-4d98-bfda-cd10214f0e55
    let client = setup_aws_client();
    let mut query = HashMap::new();

    query.insert(
        String::from(":own_id"),
        AttributeValue {
            s: Some(String::from(owner_id)),
            ..Default::default()
        },
    );

    let integrations_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            index_name: Some(String::from("OwnId")),
            key_condition_expression: Some(String::from("OwnId = :own_id")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match integrations_result {
        Ok(result) => {
            let integration_ids: Vec<String> = result
                .items
                .unwrap_or_else(|| Vec::new())
                .iter()
                .map(|item| item.get("PK").and_then(|attr| attr.s.clone()))
                .flatten()
                .collect();

            let mut all_integrations = Vec::new();

            for integration_id in integration_ids {
                println!("Integration ID: {}", integration_id);
                let integration_result = get_integration_by_id(&integration_id).await;
                all_integrations.push(integration_result.unwrap().unwrap());
            }
            Ok(all_integrations)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(())
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

pub async fn get_integration_with_mapped_fields_and_modules(
    integration_id: &str,
) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();
    let sk = format!("{}", integration_id);

    //I#01HV177W1JAS01D5J3EZDSKCC0

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
                .map(|item| {
                    let pk = item
                        .get("PK")
                        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                        .unwrap_or_else(|| panic!("PK attribute not found"));

                    let sk = item
                        .get("SK")
                        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                        .unwrap_or_else(|| panic!("SK attribute not found"));

                    let pk_f = format!("{}#F#", pk);
                    let pk_m = format!("{}#M#", pk);

                    let processed_item = if pk == sk {
                        ProcessedItem::Integration(integrations::process_integration_item(item))
                    } else if sk.contains(&pk_f) {
                        ProcessedItem::MappedField(mapped_fields::process_mapped_field_item(item))
                    } else if sk.contains(&pk_m) {
                        ProcessedItem::Module(modules::process_module_item(item))
                    } else {
                        panic!("Unknown item type");
                    };

                    processed_item
                })
                .collect::<Vec<_>>(),
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

pub async fn get_sync_by_id(sync_id: &str) -> Result<Option<Vec<Sync>>, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(sync_id)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| syncs::process_sync_item(item))
                .collect::<Vec<Sync>>(),
            None => Vec::new(),
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    if items.is_empty() {
        return Ok(None);
    } else {
        return Ok(Some(items));
    }
}

pub async fn get_sync(sync_id: &str) -> Result<String, ()> {
    let sync = get_sync_by_id(sync_id).await.unwrap();

    let serialized = serde_json::to_string_pretty(&sync).unwrap();

    Ok(serialized)
}

pub async fn get_log(log_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();

    // L#01HSV10JG7R4DRW2RFCCHTFVHP

    query.insert(
        String::from(":pk"),
        AttributeValue {
            s: Some(String::from(log_id)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| logs::process_log_item(item))
                .collect::<Vec<Log>>(),
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

pub async fn get_all_logs_for_sync(sync_id: &str) -> Result<String, ()> {
    let client = setup_aws_client();
    let mut query = HashMap::new();

    // I#01HTQ6P9MFK3V2FSD20Z1SF3VA#S#01HW2J2DS020GRYDZE4QX2Z2HN

    query.insert(
        String::from(":sk"),
        AttributeValue {
            s: Some(String::from(sync_id)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            index_name: Some(String::from("SK-LId")),
            key_condition_expression: Some(String::from("SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| logs::process_log_item(item))
                .collect::<Vec<Log>>(),
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

pub async fn get_all_syncs_for_primary_entity(primary_entity: &str) -> Result<String, ()> {
    // GeckoChat:01e4a092-ecbc-4b93-8228-2fbb1c686083:contact:28fd075b-a7ec-4344-b1f6-b077aaaa2eb0
    let client = setup_aws_client();
    let mut query = HashMap::new();

    query.insert(
        String::from(":spriid"),
        AttributeValue {
            s: Some(String::from(primary_entity)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            index_name: Some(String::from("SPriId-IId")),
            key_condition_expression: Some(String::from("SPriId = :spriid")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| syncs::process_sync_item(item))
                .collect::<Vec<Sync>>(),
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

pub async fn get_all_syncs_for_secondary_entity(secondary_entity: &str) -> Result<String, ()> {
    //
    let client = setup_aws_client();
    let mut query = HashMap::new();

    query.insert(
        String::from(":ssecid"),
        AttributeValue {
            s: Some(String::from(secondary_entity)),
            ..Default::default()
        },
    );

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            index_name: Some(String::from("SSecId-IId")),
            key_condition_expression: Some(String::from("SSecId = :ssecid")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => match result.items {
            Some(items) => items
                .iter()
                .map(|item| syncs::process_sync_item(item))
                .collect::<Vec<Sync>>(),
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
