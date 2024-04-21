extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
extern crate serde_dynamodb;
use std::env;

use crate::types::{
    AuthDetail, AuthStatus, Integration, IntegrationStatus, MappedField, PrimaryConfig,
    PrimaryConnection, SecondaryConfig, SecondaryConnection, SetupComplete,
};

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

    let integrations: Vec<Integration> = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await
        .unwrap()
        .items
        .unwrap()
        .iter()
        .map(|item| {
            let integration = Integration {
                pk: item.get("PK").unwrap().s.as_ref().unwrap().to_string(),
                sk: item.get("SK").unwrap().s.as_ref().unwrap().to_string(),
                own_id: item.get("PK").unwrap().s.as_ref().unwrap().to_string(),
                cr_at: item.get("CrAt").unwrap().s.as_ref().unwrap().to_string(),
                up_at: item.get("UpAt").unwrap().s.as_ref().unwrap().to_string(),
                pri_con: PrimaryConnection {
                    connection_type: item
                        .get("PriCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("connectionType")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                    connection_name: item
                        .get("PriCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("connectionName")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                    account_id: item
                        .get("PriCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("accountId")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                },
                sec_con: SecondaryConnection {
                    connection_name: item
                        .get("SecCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("connectionName")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                    account_id: item
                        .get("SecCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("accountId")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                    connection_type: item
                        .get("SecCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("connectionType")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                    api_domain: item
                        .get("SecCon")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("api_domain")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                },
                pri_auth: item.get("PriAuth").unwrap().s.as_ref().unwrap().to_string(),
                sec_auth: item.get("SecAuth").unwrap().s.as_ref().unwrap().to_string(),
                i_status: IntegrationStatus {
                    setup_complete: SetupComplete {
                        primary: item
                            .get("IStatus")
                            .unwrap()
                            .m
                            .as_ref()
                            .unwrap()
                            .get("setupComplete")
                            .unwrap()
                            .m
                            .as_ref()
                            .unwrap()
                            .get("primary")
                            .unwrap()
                            .bool
                            .unwrap(),
                        secondary: item
                            .get("IStatus")
                            .unwrap()
                            .m
                            .as_ref()
                            .unwrap()
                            .get("setupComplete")
                            .unwrap()
                            .m
                            .as_ref()
                            .unwrap()
                            .get("secondary")
                            .unwrap()
                            .bool
                            .unwrap(),
                    },
                    auth: AuthStatus {
                        secondary: AuthDetail {
                            code: item
                                .get("IStatus")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("auth")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("secondary")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("code")
                                .unwrap()
                                .s
                                .as_ref()
                                .unwrap()
                                .to_string(),
                        },
                        primary: AuthDetail {
                            code: item
                                .get("IStatus")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("auth")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("primary")
                                .unwrap()
                                .m
                                .as_ref()
                                .unwrap()
                                .get("code")
                                .unwrap()
                                .s
                                .as_ref()
                                .unwrap()
                                .to_string(),
                        },
                    },
                },
            };
            integration
        })
        .collect();

    let serialized = serde_json::to_string_pretty(&integrations).unwrap();

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

    let items: Vec<MappedField> = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await
        .unwrap()
        .items
        .unwrap()
        .iter()
        .map(|item| {
            let mapped_field = MappedField {
                pk: item.get("PK").unwrap().s.as_ref().unwrap().to_string(),
                sk: item.get("SK").unwrap().s.as_ref().unwrap().to_string(),
                cr_at: item.get("CrAt").unwrap().s.as_ref().unwrap().to_string(),
                f_id: item.get("FId").unwrap().s.as_ref().unwrap().to_string(),
                f_pri_id: item.get("FPriId").unwrap().s.as_ref().unwrap().to_string(),
                f_sec_id: item.get("FSecId").unwrap().s.as_ref().unwrap().to_string(),
                pri_cfg: PrimaryConfig {
                    label: item
                        .get("PriCfg")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("label")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                },
                pri_lbl: item.get("PriLbl").unwrap().s.as_ref().unwrap().to_string(),
                pri_mod: item.get("PriMod").unwrap().s.as_ref().unwrap().to_string(),
                pri_type: item.get("PriType").unwrap().s.as_ref().unwrap().to_string(),
                sec_cfg: SecondaryConfig {
                    format: item
                        .get("SecCfg")
                        .unwrap()
                        .m
                        .as_ref()
                        .unwrap()
                        .get("format")
                        .unwrap()
                        .s
                        .as_ref()
                        .unwrap()
                        .to_string(),
                },
                sec_lbl: item.get("SecLbl").unwrap().s.as_ref().unwrap().to_string(),
                sec_mod: item.get("SecMod").unwrap().s.as_ref().unwrap().to_string(),
                sec_type: item.get("SecType").unwrap().s.as_ref().unwrap().to_string(),
            };
            mapped_field
        })
        .collect();

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
