use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

use crate::types::Log;

pub fn process_log_item(item: &HashMap<String, AttributeValue>) -> Log {
    let pk = get_string_attribute(item, "PK");
    let sk = get_string_attribute(item, "SK");
    let con_cat = get_string_attribute(item, "ConCat");
    let dat_tim = get_string_attribute(item, "DatTim");
    let i_id = get_string_attribute(item, "IId");
    let l_id = get_string_attribute(item, "LId");
    let l_s_id = get_string_attribute(item, "LSId");
    let req = get_string_attribute(item, "Req");
    let res = get_string_attribute(item, "Res");
    let ttl = get_string_attribute(item, "TTL");

    Log {
        pk,
        sk,
        con_cat,
        dat_tim,
        i_id,
        l_id,
        l_s_id,
        req,
        res,
        ttl,
    }
}

fn get_string_attribute(item: &HashMap<String, AttributeValue>, key: &str) -> String {
    item.get(key)
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from(""))
}
