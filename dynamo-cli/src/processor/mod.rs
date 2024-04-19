use crate::dynamo::ListItemsResult;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamodb;
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
    return "ffs";
}
