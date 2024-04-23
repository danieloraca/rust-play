use crate::types::{
    AuthDetail, AuthStatus, Integration, IntegrationStatus, PrimaryConnection, SecondaryConnection,
    SetupComplete,
};
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub fn process_integration_item(item: &HashMap<String, AttributeValue>) -> Integration {
    let pk = item
        .get("PK")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("PK attribute not found"));

    let sk = item
        .get("SK")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("SK attribute not found"));

    let own_id = item
        .get("PK")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("OwnId attribute not found"));

    let cr_at = item
        .get("CrAt")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("CrAt attribute not found"));

    let up_at = item
        .get("UpAt")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("UpAt attribute not found"));

    let pri_con = PrimaryConnection {
        connection_type: item
            .get("PriCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("connectionType"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("PriCon connectionType attribute not found")),

        connection_name: item
            .get("PriCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("connectionName"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("PriCon connectionName attribute not found")),

        account_id: item
            .get("PriCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("accountId"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("PriCon accountId attribute not found")),
    };

    let sec_con = SecondaryConnection {
        connection_name: item
            .get("SecCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("connectionName"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("SecCon connectionName attribute not found")),

        account_id: item
            .get("SecCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("accountId"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("SecCon accountId attribute not found")),

        connection_type: item
            .get("SecCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("connectionType"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| panic!("SecCon connectionType attribute not found")),

        api_domain: item
            .get("SecCon")
            .and_then(|attr| attr.m.as_ref())
            .and_then(|m| m.get("api_domain"))
            .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| String::new()),
    };

    let pri_auth = item
        .get("PriAuth")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("PriAuth attribute not found"));

    let sec_auth = item
        .get("SecAuth")
        .and_then(|attr| attr.s.as_ref().map(|s| s.to_string()))
        .unwrap_or_else(|| panic!("SecAuth attribute not found"));

    let i_status = IntegrationStatus {
        setup_complete: SetupComplete {
            primary: item
                .get("IStatus")
                .and_then(|attr| attr.m.as_ref())
                .and_then(|m| m.get("setupComplete"))
                .and_then(|attr| attr.m.as_ref())
                .and_then(|m| m.get("primary"))
                .and_then(|attr| attr.bool)
                .unwrap_or_else(|| panic!("IStatus setupComplete primary attribute not found")),

            secondary: item
                .get("IStatus")
                .and_then(|attr| attr.m.as_ref())
                .and_then(|m| m.get("setupComplete"))
                .and_then(|attr| attr.m.as_ref())
                .and_then(|m| m.get("secondary"))
                .and_then(|attr| attr.bool)
                .unwrap_or_else(|| panic!("IStatus setupComplete secondary attribute not found")),
        },
        auth: AuthStatus {
            primary: AuthDetail {
                code: item
                    .get("IStatus")
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("auth"))
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("primary"))
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("code"))
                    .and_then(|attr| attr.s.as_ref())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| panic!("IStatus auth primary code attribute not found")),
            },
            secondary: AuthDetail {
                code: item
                    .get("IStatus")
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("auth"))
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("secondary"))
                    .and_then(|attr| attr.m.as_ref())
                    .and_then(|m| m.get("code"))
                    .and_then(|attr| attr.s.as_ref())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| panic!("IStatus auth secondary code attribute not found")),
            },
        },
    };

    Integration {
        pk,
        sk,
        own_id,
        cr_at,
        up_at,
        pri_con,
        sec_con,
        pri_auth,
        sec_auth,
        i_status,
    }
}
