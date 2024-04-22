extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
extern crate serde_dynamodb;
use std::env;

use crate::types::{
    AuthDetail, AuthStatus, Integration, IntegrationStatus, MappedField, Module, PrimaryConfig,
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

    let integrations_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let integrations = match integrations_result {
        Ok(result) => {
            match result.items {
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

                        let own_id = item
                            .get("PK")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("OwnId attribute not found"));

                        let cr_at = item
                            .get("CrAt")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("CrAt attribute not found"));

                        let up_at = item
                            .get("UpAt")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("UpAt attribute not found"));

                        let pri_con = PrimaryConnection {
                            connection_type: item
                                .get("PriCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("connectionType"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| {
                                    panic!("PriCon connectionType attribute not found")
                                }),

                            connection_name: item
                                .get("PriCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("connectionName"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| {
                                    panic!("PriCon connectionName attribute not found")
                                }),

                            account_id: item
                                .get("PriCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("accountId"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("PriCon accountId attribute not found")),
                        };

                        let sec_con = SecondaryConnection {
                            connection_name: item
                                .get("SecCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("connectionName"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| {
                                    panic!("SecCon connectionName attribute not found")
                                }),

                            account_id: item
                                .get("SecCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("accountId"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SecCon accountId attribute not found")),

                            connection_type: item
                                .get("SecCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("connectionType"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| {
                                    panic!("SecCon connectionType attribute not found")
                                }),

                            api_domain: item
                                .get("SecCon")
                                .and_then(|attr| attr.m.as_ref())
                                .and_then(|m| m.get("api_domain"))
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SecCon api_domain attribute not found")),
                        };

                        let pri_auth = item
                            .get("PriAuth")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("PriAuth attribute not found"));

                        let sec_auth = item
                            .get("SecAuth")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("SecAuth attribute not found"));

                        let i_status = IntegrationStatus {
                            setup_complete: SetupComplete {
                                primary: item
                                    .get("IStatus")
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("setupComplete"))
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("primary"))
                                    .and_then(|attr| attr.bool)
                                    .unwrap_or_else(|| {
                                        panic!("IStatus setupComplete primary attribute not found")
                                    }),

                                secondary: item
                                    .get("IStatus")
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("setupComplete"))
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("secondary"))
                                    .and_then(|attr| attr.bool)
                                    .unwrap_or_else(|| {
                                        panic!(
                                            "IStatus setupComplete secondary attribute not found"
                                        )
                                    }),
                            },
                            auth: AuthStatus {
                                primary: AuthDetail {
                                    code: item
                                        .get("IStatus")
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("auth"))
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("primary"))
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("code"))
                                        .and_then(|attr| attr.s.as_ref())
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| {
                                            panic!("IStatus auth primary code attribute not found")
                                        }),
                                },
                                secondary: AuthDetail {
                                    code: item
                                        .get("IStatus")
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("auth"))
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("secondary"))
                                        .and_then(|attr| attr.m.as_ref())
                                        .and_then(|m| m.get("code"))
                                        .and_then(|attr| attr.s.as_ref())
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "IStatus auth secondary code attribute not found"
                                            )
                                        }),
                                },
                            },
                        };

                        Integration {
                            pk,
                            sk,
                            own_id,
                            cr_at,
                            up_at,
                            pri_con,
                            sec_con,
                            pri_auth,
                            sec_auth,
                            i_status,
                        }
                    })
                    .collect::<Vec<Integration>>(),
                None => {
                    Vec::new() // or handle this case as you see fit
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new() // or handle this error case as you see fit
        }
    };

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

    let items_result = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and SK = :sk")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await;

    let items = match items_result {
        Ok(result) => {
            match result.items {
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

                        let cr_at = item
                            .get("CrAt")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("CrAt attribute not found"));

                        let f_id = item
                            .get("FId")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("FId attribute not found"));

                        let f_pri_id = item
                            .get("FPriId")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("FPriId attribute not found"));

                        let f_sec_id = item
                            .get("FSecId")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("FSecId attribute not found"));

                        let pri_cfg = item
                            .get("PriCfg")
                            .and_then(|attr| attr.m.as_ref())
                            .and_then(|m| m.get("label"))
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("PriCfg label attribute not found"));

                        let pri_lbl = item
                            .get("PriLbl")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("PriLbl attribute not found"));

                        let pri_mod = item
                            .get("PriMod")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("PriMod attribute not found"));

                        let pri_type = item
                            .get("PriType")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("PriType attribute not found"));

                        let sec_cfg = item
                            .get("SecCfg")
                            .and_then(|attr| attr.m.as_ref())
                            .and_then(|m| m.get("format"))
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("SecCfg format attribute not found"));

                        let sec_lbl = item
                            .get("SecLbl")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("SecLbl attribute not found"));

                        let sec_mod = item
                            .get("SecMod")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("SecMod attribute not found"));

                        let sec_type = item
                            .get("SecType")
                            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                            .unwrap_or_else(|| panic!("SecType attribute not found"));

                        MappedField {
                            pk,
                            sk,
                            cr_at,
                            f_id,
                            f_pri_id,
                            f_sec_id,
                            pri_cfg: PrimaryConfig { label: pri_cfg },
                            pri_lbl,
                            pri_mod,
                            pri_type,
                            sec_cfg: SecondaryConfig { format: sec_cfg },
                            sec_lbl,
                            sec_mod,
                            sec_type,
                        }
                    })
                    .collect::<Vec<MappedField>>(),
                None => {
                    Vec::new() // or handle this case as you see fit
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new() // or handle this error case as you see fit
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
        Ok(result) => {
            match result.items {
                Some(items) => items
                    .iter()
                    .map(|item| {
                        let mapped_field = MappedField {
                            pk: item
                                .get("PK")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("PK attribute not found")),
                            sk: item
                                .get("SK")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SK attribute not found")),
                            cr_at: item
                                .get("CrAt")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("CrAt attribute not found")),
                            f_id: item
                                .get("FId")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("FId attribute not found")),
                            f_pri_id: item
                                .get("FPriId")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("FPriId attribute not found")),
                            f_sec_id: item
                                .get("FSecId")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("FSecId attribute not found")),
                            pri_cfg: PrimaryConfig {
                                label: item
                                    .get("PriCfg")
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("label"))
                                    .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                    .unwrap_or_else(|| panic!("PriCfg label attribute not found")),
                            },
                            pri_lbl: item
                                .get("PriLbl")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("PriLbl attribute not found")),
                            pri_mod: item
                                .get("PriMod")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("PriMod attribute not found")),
                            pri_type: item
                                .get("PriType")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("PriType attribute not found")),
                            sec_cfg: SecondaryConfig {
                                format: item
                                    .get("SecCfg")
                                    .and_then(|attr| attr.m.as_ref())
                                    .and_then(|m| m.get("format"))
                                    .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                    .unwrap_or_else(|| panic!("SecCfg format attribute not found")),
                            },
                            sec_lbl: item
                                .get("SecLbl")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SecLbl attribute not found")),
                            sec_mod: item
                                .get("SecMod")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SecMod attribute not found")),
                            sec_type: item
                                .get("SecType")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_else(|| panic!("SecType attribute not found")),
                        };
                        mapped_field
                    })
                    .collect::<Vec<MappedField>>(),
                None => {
                    Vec::new() // or handle this case as you see fit
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new() // or handle this error case as you see fit
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
        Ok(result) => {
            match result.items {
                Some(items) => items
                    .iter()
                    .map(|item| {
                        let module = Module {
                            pk: item
                                .get("PK")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            sk: item
                                .get("SK")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            cr_at: item
                                .get("CrAt")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            con_cat: item
                                .get("ConCat")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            hdl: item
                                .get("Hdl")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            lbl: item
                                .get("Lbl")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                            m_id: item
                                .get("MId")
                                .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                                .unwrap_or_default(),
                        };
                        let module = module;
                        let module = module;
                        module
                    })
                    .collect::<Vec<Module>>(),
                None => {
                    Vec::new() // or handle this case as you see fit
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new() // or handle this error case as you see fit
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

    let items: Vec<Module> = client
        .query(QueryInput {
            table_name: String::from("Stage-Integrations"),
            key_condition_expression: Some(String::from("PK = :pk and begins_with(SK, :sk)")),
            expression_attribute_values: Some(query),
            ..Default::default()
        })
        .await
        .unwrap()
        .items
        .unwrap()
        .iter()
        .map(|item| {
            let module = Module {
                pk: item.get("PK").unwrap().s.as_ref().unwrap().to_string(),
                sk: item.get("SK").unwrap().s.as_ref().unwrap().to_string(),
                cr_at: item.get("CrAt").unwrap().s.as_ref().unwrap().to_string(),
                con_cat: item.get("ConCat").unwrap().s.as_ref().unwrap().to_string(),
                hdl: item.get("Hdl").unwrap().s.as_ref().unwrap().to_string(),
                lbl: item.get("Lbl").unwrap().s.as_ref().unwrap().to_string(),
                m_id: item.get("MId").unwrap().s.as_ref().unwrap().to_string(),
            };
            module
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
