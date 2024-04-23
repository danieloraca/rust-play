use crate::types::{SResult, Sync, SyncError, SyncResultData};
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub fn process_sync_item(item: &HashMap<String, AttributeValue>) -> Sync {
    let pk = get_string_attribute(item, "PK");
    let sk = get_string_attribute(item, "SK");
    let cr_at = get_string_attribute(item, "CrAt");
    let i_id = get_string_attribute(item, "IId");
    let s_id = get_string_attribute(item, "SId");
    let s_pri_id = get_string_attribute(item, "SPriId");
    let s_pri_mod = get_string_attribute(item, "SPriMod");
    let s_sec_id = get_string_attribute(item, "SSecId"); // Optional field
    let s_sec_mod = get_string_attribute(item, "SSecMod");
    let s_status = get_string_attribute(item, "SStatus");
    let up_at = get_string_attribute(item, "UpAt");

    let s_result = if let Some(result_attr) = item.get("SResult").and_then(|attr| attr.m.as_ref()) {
        let data_attr = result_attr.get("data").and_then(|attr| attr.m.as_ref());

        let error = data_attr
            .and_then(|m| m.get("error"))
            .and_then(|attr| attr.s.as_ref())
            .map_or(String::new(), |s| s.to_string());

        let sync_error = SyncError { sync_error: error };

        let secondary_id = result_attr
            .get("secondaryId")
            .and_then(|attr| attr.s.as_ref())
            .map_or(String::new(), |s| s.to_string());

        let sync_status = result_attr
            .get("syncStatus")
            .and_then(|attr| attr.s.as_ref())
            .map_or(String::new(), |s| s.to_string());

        SResult {
            data: SyncResultData { error: sync_error },
            secondary_id,
            sync_status,
        }
    } else {
        SResult {
            data: SyncResultData {
                error: SyncError {
                    sync_error: String::new(),
                },
            },
            secondary_id: String::new(),
            sync_status: String::new(),
        }
    };

    // let pri_con = PrimaryConnection {
    //     connection_type: item
    //         .get("PriCon")
    //         .and_then(|attr| attr.m.as_ref())
    //         .and_then(|m| m.get("connectionType"))
    //         .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
    //         .unwrap_or_else(|| panic!("PriCon connectionType attribute not found")),
    // };

    Sync {
        pk,
        sk,
        cr_at,
        i_id,
        s_id,
        s_pri_id,
        s_pri_mod,
        s_sec_id,
        s_sec_mod,
        s_status,
        s_result,
        up_at,
    }
}

fn get_string_attribute(item: &HashMap<String, AttributeValue>, key: &str) -> String {
    item.get(key)
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from(""))
}
