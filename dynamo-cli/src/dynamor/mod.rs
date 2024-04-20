extern crate serde;
extern crate serde_dynamodb;

extern crate rusoto_core;
extern crate rusoto_dynamodb;
use serde_json::{from_str, Value};
// #[macro_use]
// extern crate serde_derive;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Integration {
    pk: String,
    sk: String,
    own_id: String,
    cr_at: String,
    up_at: String,
    pri_con: PrimaryConnection,
    sec_con: SecondaryConnection,
    pri_auth: String,
    sec_auth: String,
    i_status: IntegrationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryConnection {
    #[serde(rename = "ConnectionType")]
    connection_type: String,
    connection_name: String,
    account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondaryConnection {
    connection_name: String,
    account_id: String,
    #[serde(rename = "ConnectionType")]
    connection_type: String,
    api_domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationStatus {
    setup_complete: SetupComplete,
    auth: AuthStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupComplete {
    primary: bool,
    secondary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStatus {
    secondary: AuthDetail,
    primary: AuthDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthDetail {
    code: String,
}

pub async fn get_integration(integration_id: &str) -> Result<Integration, ()> {
    println!("Hello, world! {:?}", integration_id);

    let client = DynamoDbClient::new(Region::EuWest1);
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

    let integration = Integration {
        pk: integrations[0].pk.clone(),
        sk: integrations[0].sk.clone(),
        own_id: integrations[0].own_id.clone(),
        cr_at: integrations[0].cr_at.clone(),
        up_at: integrations[0].up_at.clone(),
        pri_con: PrimaryConnection {
            connection_type: integrations[0].pri_con.connection_type.clone(),
            connection_name: integrations[0].pri_con.connection_name.clone(),
            account_id: integrations[0].pri_con.account_id.clone(),
        },
        sec_con: SecondaryConnection {
            connection_name: integrations[0].sec_con.connection_name.clone(),
            account_id: integrations[0].sec_con.account_id.clone(),
            connection_type: integrations[0].sec_con.connection_type.clone(),
            api_domain: integrations[0].sec_con.api_domain.clone(),
        },
        pri_auth: integrations[0].pri_auth.clone(),
        sec_auth: integrations[0].sec_auth.clone(),
        i_status: IntegrationStatus {
            setup_complete: SetupComplete {
                primary: integrations[0].i_status.setup_complete.primary,
                secondary: integrations[0].i_status.setup_complete.secondary,
            },
            auth: AuthStatus {
                secondary: AuthDetail {
                    code: integrations[0].i_status.auth.secondary.code.clone(),
                },
                primary: AuthDetail {
                    code: integrations[0].i_status.auth.primary.code.clone(),
                },
            },
        },
    };

    Ok(integration)
}
