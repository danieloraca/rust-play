use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() {
    loop {
        let options = vec!["Option 1", "Option 2", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0) // Set default selection to Option 1
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("You selected Option 1!");
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter some text:")
                    .interact()
                    .unwrap();
                println!("You entered: {}", input);
            }
            1 => {
                println!("You selected Option 2!");
                // Place your code for Option 2 here
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
