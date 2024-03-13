use std::io;
mod solar_systems;
mod dynamo_db;

// struct Config {
//     pattern: String,
//     path: String,
// }

fn main() {
    loop {
        println!("Please select an option:");
        println!("1. Create a solar system");
        println!("2. View a solar system");
        println!("3. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                println!("Creating a solar system");
                solar_systems::add_solar_system()
            }
            2 => {
                println!("Viewing a solar system");
            }
            3 => {
                println!("Exiting");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}
