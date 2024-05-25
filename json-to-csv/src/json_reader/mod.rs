use crate::types::Attendances;
use crate::types::Data;
use crate::types::JsonData;
use std::fs::File;
use std::io::Read;

pub fn read_json_file(file_name: &str) -> Vec<Attendances> {
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let parsed_data: JsonData = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let data: Data = parsed_data.data;
    let attendances: Vec<Attendances> = data.attendances;

    attendances
}
