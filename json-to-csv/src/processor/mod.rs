use crate::types::Attendances;
use crate::types::CsvData;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;

pub fn write_csv(file_path: &str, data: &[CsvData]) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn get_full_name_and_email_by_attendance_id(
    attendances: &Vec<Attendances>,
    attendance_id: i32,
) -> (String, String) {
    let mut full_name = String::new();
    let mut email = String::new();
    attendances.iter().for_each(|data| {
        if data.id == attendance_id {
            full_name = data.name.clone();
            email = data.email.clone();
        }
    });

    (full_name, email)
}
