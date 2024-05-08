use serde_json::{self, Value};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Get the JSON file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_file>", args[0]);
        std::process::exit(1);
    }
    let json_file_path = &args[1];

    // Read the JSON file content
    let mut file = File::open(json_file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    // Parse JSON content into a serde_json::Value
    let json_value: Value = serde_json::from_str(&json_content)?;

    // Convert snake case keys to camel case
    let json_camelcase = convert_to_camelcase(&json_value)?;

    // Write the modified JSON to a new file
    let output_file_path = format!("{}_camelcase.json", json_file_path);
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(serde_json::to_string_pretty(&json_camelcase)?.as_bytes())?;

    // println!("Converted JSON saved to {}", output_file_path.clone());
    Ok(())
}

fn convert_to_camelcase(value: &Value) -> io::Result<Value> {
    match value {
        Value::Object(obj) => {
            let mut new_obj = serde_json::Map::new();
            for (key, val) in obj.iter() {
                let new_key = to_camelcase(key)?;
                let new_val = convert_to_camelcase(val)?;
                new_obj.insert(new_key, new_val);
            }
            Ok(Value::Object(new_obj))
        }
        Value::Array(arr) => {
            let mut new_arr = Vec::new();
            for val in arr.iter() {
                new_arr.push(convert_to_camelcase(val)?);
            }
            Ok(Value::Array(new_arr))
        }
        other => Ok(other.clone()), // Preserve other types as is
    }
}

fn to_camelcase(snake_case: &str) -> io::Result<String> {
    let mut camelcase = String::new();
    let mut capitalize_next = false;
    for c in snake_case.chars() {
        match c {
            '_' => capitalize_next = true,
            _ if capitalize_next => {
                camelcase.push(c.to_ascii_uppercase());
                capitalize_next = false;
            }
            _ => camelcase.push(c),
        }
    }
    Ok(camelcase)
}
