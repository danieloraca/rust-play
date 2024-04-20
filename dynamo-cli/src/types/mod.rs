use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Integration {
    pub pk: String,
    pub sk: String,
    pub own_id: String,
    pub cr_at: String,
    pub up_at: String,
    pub pri_con: PrimaryConnection,
    pub sec_con: SecondaryConnection,
    pub pri_auth: String,
    pub sec_auth: String,
    pub i_status: IntegrationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryConnection {
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    pub connection_name: String,
    pub account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondaryConnection {
    pub connection_name: String,
    pub account_id: String,
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    pub api_domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationStatus {
    pub setup_complete: SetupComplete,
    pub auth: AuthStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupComplete {
    pub primary: bool,
    pub secondary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStatus {
    pub secondary: AuthDetail,
    pub primary: AuthDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthDetail {
    pub code: String,
}
