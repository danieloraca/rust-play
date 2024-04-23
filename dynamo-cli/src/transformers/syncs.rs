use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

use crate::types::{SResult, Sync, SyncResultData};

pub fn process_sync_item(item: &HashMap<String, AttributeValue>) -> Sync {
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

    let i_id = item
        .get("IId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("IId attribute not found"));

    let s_id = item
        .get("SId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("SId attribute not found"));

    let s_pri_id = item
        .get("SPriId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::new());

    let s_pri_mod = item
        .get("SPriMod")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::new());

    let s_sec_id = item
        .get("SSecId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::new());

    let s_sec_mod = item
        .get("SSecMod")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::new());

    let s_status = item
        .get("SStatus")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("SStatus attribute not found"));

    let s_result = SResult {
        data: item
            .get("SResult")
            .and_then(|attr| attr.m.as_ref())
            .map(|m| {
                let data = m
                    .get("data")
                    .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
                    .unwrap_or_else(|| String::new());

                SyncResultData { data }
            })
            .unwrap_or_else(|| panic!("SResult attribute not found")),
        secondary_id: item
            .get("SResult")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| {
                m.get("secondaryId")
                    .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| panic!("secondaryId attribute not found")),
        sync_status: item
            .get("SResult")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| {
                m.get("syncStatus")
                    .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| panic!("syncStatus attribute not found")),
    };

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
    }
}
