use crate::dynamor;
use colored::*;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use tokio::runtime::Runtime;

pub fn show_menu() {
    let rt = Runtime::new().unwrap();

    loop {
        let options = vec!["Fake", "Get Integration by ID", "Get a MappedField", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0) // Default to first option
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("{}", "Get Integration (FAKE)!".green());
                // let integration_id_hardcoded: &str = "I#01HV177W1JAS01D5J3EZDSKCC0";
                // let result =
                //     rt.block_on(async { dynamo::get_integration(integration_id_hardcoded).await });

                // match result {
                //     Ok(result) => {
                //         println!("Result: {:?}", result);
                //     }
                //     Err(e) => {
                //         println!("Error: {:?}", e);
                //     }
                // }
            }
            1 => {
                println!("Get Integration!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let result =
                    rt.block_on(async { dynamor::get_integration(integration_id.as_str()).await });
                match result {
                    Ok(result) => {
                        println!("Integration id {} is {}", integration_id, result.cyan());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            2 => {
                println!("Get a MappedField!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let mapped_field_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("MappedField ID:")
                    .interact()
                    .unwrap();
                let result = rt.block_on(async {
                    dynamor::get_mapped_field(integration_id.as_str(), mapped_field_id.as_str())
                        .await
                });
                match result {
                    Ok(result) => {
                        println!("MappedField id {} is {}", mapped_field_id, result.cyan());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => unreachable!(),
        }
        println!(); // Add an empty line for better readability
    }
}
