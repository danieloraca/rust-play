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

#[derive(Serialize, Deserialize, Debug)]
pub struct MappedField {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "CrAt")]
    pub cr_at: String,
    #[serde(rename = "FId")]
    pub f_id: String,
    #[serde(rename = "FPriId")]
    pub f_pri_id: String,
    #[serde(rename = "FSecId")]
    pub f_sec_id: String,
    #[serde(rename = "PriCfg")]
    pub pri_cfg: PrimaryConfig,
    #[serde(rename = "PriLbl")]
    pub pri_lbl: String,
    #[serde(rename = "PriMod")]
    pub pri_mod: String,
    #[serde(rename = "PriType")]
    pub pri_type: String,
    #[serde(rename = "SecCfg")]
    pub sec_cfg: SecondaryConfig,
    #[serde(rename = "SecLbl")]
    pub sec_lbl: String,
    #[serde(rename = "SecMod")]
    pub sec_mod: String,
    #[serde(rename = "SecType")]
    pub sec_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryConfig {
    #[serde(rename = "label")]
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondaryConfig {
    #[serde(rename = "format")]
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "CrAt")]
    pub cr_at: String,
    #[serde(rename = "ConCat")]
    pub con_cat: String,
    #[serde(rename = "Hdl")]
    pub hdl: String,
    #[serde(rename = "Lbl")]
    pub lbl: String,
    #[serde(rename = "MId")]
    pub m_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sync {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "CrAt")]
    pub cr_at: String,
    #[serde(rename = "IId")]
    pub i_id: String,
    #[serde(rename = "SId")]
    pub s_id: String,
    #[serde(rename = "SPriId")]
    pub s_pri_id: String,
    #[serde(rename = "SPriMod")]
    pub s_pri_mod: String,
    #[serde(rename = "SSecId")]
    pub s_sec_id: String,
    #[serde(rename = "SSecMod")]
    pub s_sec_mod: String,
    #[serde(rename = "SStatus")]
    pub s_status: String,
    #[serde(rename = "SResult")]
    pub s_result: SResult,
    #[serde(rename = "UpAt")]
    pub up_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SResult {
    #[serde(rename = "data")]
    pub data: SyncResultData,
    #[serde(rename = "secondaryId")]
    pub secondary_id: String,
    #[serde(rename = "syncStatus")]
    pub sync_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncResultData {
    #[serde(rename = "error")]
    pub error: SyncError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncError {
    #[serde(rename = "error")]
    pub sync_error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "ConCat")]
    pub con_cat: String,
    #[serde(rename = "DatTim")]
    pub dat_tim: String,
    #[serde(rename = "IId")]
    pub i_id: String,
    #[serde(rename = "LId")]
    pub l_id: String,
    #[serde(rename = "LSId")]
    pub l_s_id: String,
    #[serde(rename = "Req")]
    pub req: String,
    #[serde(rename = "Res")]
    pub res: String,
    #[serde(rename = "TTL")]
    pub ttl: String,
}
