use crate::dynamor;
use colored::*;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use tokio::runtime::Runtime;

pub fn show_menu() {
    let rt = Runtime::new().unwrap();

    loop {
        let options = vec![
            "Get Integration by ID",
            "Get a MappedField",
            "Get all MappedFields for an Integration",
            "Get a Module",
            "Get all Modules for an Integration",
            "Get an Integration with all its MappedFields and Modules",
            "Get a Sync",
            "Get a Log",
            "Get all Integrations for an Owner",
            "Get all Syncs for a Primary Entity",
            "Get all Syncs for a Secondary Entity",
            "Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0) // Default to first option
            .interact()
            .unwrap();

        match selection {
            0 => {
                // I#01HV177W1JAS01D5J3EZDSKCC0
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
            1 => {
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
            2 => {
                println!("Get all MappedFields for an Integration!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let result = rt.block_on(async {
                    dynamor::get_all_mapped_fields_for_integration(integration_id.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "MappedFields for integration id {} are {}",
                            integration_id,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            3 => {
                println!("Get a Module!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let module_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Module ID:")
                    .interact()
                    .unwrap();
                let result = rt.block_on(async {
                    dynamor::get_module(integration_id.as_str(), module_id.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!("Module id {} is {}", module_id, result.cyan());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            4 => {
                println!("Get all Modules for an Integration!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let result = rt.block_on(async {
                    dynamor::get_all_modules_for_integration(integration_id.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "Modules for integration id {} are {}",
                            integration_id,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            5 => {
                println!("Get an Integration with all its MappedFields and Modules!");
                let integration_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Integration ID:")
                    .interact()
                    .unwrap();
                let result = rt.block_on(async {
                    dynamor::get_integration_with_mapped_fields_and_modules(integration_id.as_str())
                        .await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "Integration with id {} is {}",
                            integration_id,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            6 => {
                println!("Get a Sync!");
                let sync_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Sync ID:")
                    .interact()
                    .unwrap();

                let result = rt.block_on(async { dynamor::get_sync(sync_id.as_str()).await });
                match result {
                    Ok(result) => {
                        println!("Sync id {} is {}", sync_id, result.cyan());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            7 => {
                println!("Get a Log!");
                let log_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Log ID:")
                    .interact()
                    .unwrap();

                let result = rt.block_on(async { dynamor::get_log(log_id.as_str()).await });
                match result {
                    Ok(result) => {
                        println!("Log id {} is {}", log_id, result.cyan());
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            8 => {
                println!("Get all Integrations for an Owner!");
                let owner_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Owner ID:")
                    .interact()
                    .unwrap();

                let result = rt.block_on(async {
                    dynamor::get_all_integrations_for_owner(owner_id.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "Integrations for owner id {} are {}",
                            owner_id,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            9 => {
                println!("Get all Syncs for a Primary Entity!");
                let primary_entity: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("SPriId:")
                    .interact()
                    .unwrap();

                let result = rt.block_on(async {
                    dynamor::get_all_syncs_for_primary_entity(primary_entity.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "Syncs for primary entity {} are {}",
                            primary_entity,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            10 => {
                println!("Get all Syncs for a Secondary Entity!");
                let secondary_entity: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("SSecId:")
                    .interact()
                    .unwrap();

                let result = rt.block_on(async {
                    dynamor::get_all_syncs_for_secondary_entity(secondary_entity.as_str()).await
                });
                match result {
                    Ok(result) => {
                        println!(
                            "Syncs for secondary entity {} are {}",
                            secondary_entity,
                            result.cyan()
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            11 => {
                println!("Exiting...");
                break;
            }
            _ => unreachable!(),
        }
        println!(); // Add an empty line for better readability
    }
}
