use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Integration {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "OwnId")]
    pub own_id: String,
    #[serde(rename = "CrAt")]
    pub cr_at: String,
    #[serde(rename = "UpAt")]
    pub up_at: String,
    #[serde(rename = "PriCon")]
    pub pri_con: PrimaryConnection,
    #[serde(rename = "SecCon")]
    pub sec_con: SecondaryConnection,
    #[serde(rename = "PriAuth")]
    pub pri_auth: String,
    #[serde(rename = "SecAuth")]
    pub sec_auth: String,
    #[serde(rename = "IStatus")]
    pub i_status: IntegrationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryConnection {
    #[serde(rename = "connectionType")]
    pub connection_type: String,
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondaryConnection {
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "connectionType")]
    pub connection_type: String,
    pub api_domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationStatus {
    #[serde(rename = "setupComplete")]
    pub setup_complete: SetupComplete,
    #[serde(rename = "auth")]
    pub auth: AuthStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupComplete {
    #[serde(rename = "primary")]
    pub primary: bool,
    #[serde(rename = "secondary")]
    pub secondary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStatus {
    #[serde(rename = "primary")]
    pub primary: AuthDetail,
    #[serde(rename = "secondary")]
    pub secondary: AuthDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthDetail {
    pub code: String,
}
