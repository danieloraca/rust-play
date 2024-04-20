use crate::dynamo;
use crate::dynamor;
use crate::processor;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use tokio::runtime::Runtime;

pub fn show_menu() {
    let rt = Runtime::new().unwrap();

    loop {
        let options = vec!["Get Integration", "Option 2", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0) // Default to first option
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("Get Integration!");
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter integration id:")
                    .interact()
                    .unwrap();
                let integration_id_hardcoded: &str = "I#01HV177W1JAS01D5J3EZDSKCC0";
                let result =
                    rt.block_on(async { dynamo::get_integration(integration_id_hardcoded).await });
                // let input_clone = input.clone();
                // let result =
                //     rt.block_on(async { dynamo::get_integration(input_clone.as_str()).await });

                match result {
                    Ok(result) => {
                        let integration = processor::process_integration(&result);
                        println!("Result: {:?}", result);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            1 => {
                println!("You selected Option 2!");
                let result = rt.block_on(async {
                    dynamor::get_integration("I#01HV177W1JAS01D5J3EZDSKCC0").await
                });
                match result {
                    Ok(result) => {
                        println!("Result: {:?}", result);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            2 => {
                println!("Exiting...");
                break; // Exit the loop and program
            }
            _ => unreachable!(),
        }
        println!(); // Add an empty line for better readability
    }
}
