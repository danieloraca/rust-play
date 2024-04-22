use crate::types::Module;
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub fn process_module_item(item: &HashMap<String, AttributeValue>) -> Module {
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

    module
}
