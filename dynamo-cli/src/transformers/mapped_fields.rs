use crate::types::{MappedField, PrimaryConfig, SecondaryConfig};
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub fn process_mapped_field_item(item: &HashMap<String, AttributeValue>) -> MappedField {
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
        .unwrap_or_else(|| String::from("FId attribute not found"));

    let f_pri_id = item
        .get("FPriId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("FPriId attribute not found"));

    let f_sec_id = item
        .get("FSecId")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("FSecId attribute not found"));

    let pri_cfg = item
        .get("PriCfg")
        .and_then(|attr| attr.m.as_ref())
        .and_then(|m| m.get("label"))
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("PriCfg label attribute not found"));

    let pri_lbl = item
        .get("PriLbl")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("PriLbl attribute not found"));

    let pri_mod = item
        .get("PriMod")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("PriMod attribute not found"));

    let pri_type = item
        .get("PriType")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("PriType attribute not found"));

    let sec_cfg = item
        .get("SecCfg")
        .and_then(|attr| attr.m.as_ref())
        .and_then(|m| m.get("format"))
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("SecCfg format attribute not found"));

    let sec_lbl = item
        .get("SecLbl")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("SecLbl attribute not found"));

    let sec_mod = item
        .get("SecMod")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("SecMod attribute not found"));

    let sec_type = item
        .get("SecType")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("SecType attribute not found"));

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
}
