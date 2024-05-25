use crate::types::Attendances;
use csv::WriterBuilder;
use std::fs::File;
use types::CsvData;
mod csv_reader;
mod json_reader;
mod types;
use std::error::Error;

fn main() {
    let json_file_name = "raw.json";
    let csv_file_name = "sync_queue.csv";
    let attendances = json_reader::read_json_file(json_file_name);

    let mut new_vec_csv_data: Vec<CsvData> = Vec::new();
    let csv_content = csv_reader::read_csv_file(csv_file_name);
    match csv_content {
        Ok(csv_data) => {
            csv_data.iter().for_each(|data| {
                let full_name = get_full_name(&attendances, data.attendee_id);
                let email = get_email(&attendances, data.attendee_id);

                let new_csv_data = CsvData::new(
                    data.attendee_id,
                    data.event_id.clone(),
                    data.entity_id.clone(),
                    data.contact_id.clone(),
                    data.status.clone(),
                    data.metadata.clone(),
                    data.created_at,
                    Some(full_name.to_string()),
                    Some(email.to_string()),
                );
                new_vec_csv_data.push(new_csv_data);
            });
        }
        Err(e) => println!("Error: {}", e),
    }

    // new_vec_csv_data.iter().for_each(|data| {
    //     println!("{:?}", data);
    // });

    if let Err(e) = write_csv("sync_queue_new.csv", &new_vec_csv_data) {
        println!("Error: {}", e);
    }
}

fn write_csv(file_path: &str, data: &[CsvData]) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn get_full_name(attendances: &Vec<Attendances>, attendance_id: i32) -> String {
    let mut full_name = String::new();
    attendances.iter().for_each(|data| {
        if data.id == attendance_id {
            full_name = data.name.clone();
        }
    });

    full_name
}

fn get_email(attendances: &Vec<Attendances>, attendance_id: i32) -> String {
    let mut email = String::new();
    attendances.iter().for_each(|data| {
        if data.id == attendance_id {
            email = data.email.clone();
        }
    });

    email
}
