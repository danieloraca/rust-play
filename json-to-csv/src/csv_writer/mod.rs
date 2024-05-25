use std::error::Error;

use crate::{
    processor,
    types::{Attendances, CsvData},
};

pub fn write_to_csv(
    attendances: &Vec<Attendances>,
    csv_content: &Result<Vec<CsvData>, Box<dyn Error>>,
) {
    let mut new_vec_csv_data: Vec<CsvData> = Vec::new();
    match csv_content {
        Ok(csv_data) => {
            csv_data.iter().for_each(|data| {
                let (full_name, email) = processor::get_full_name_and_email_by_attendance_id(
                    &attendances,
                    data.attendee_id,
                );

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

    if let Err(e) = processor::write_csv("sync_queue_new.csv", &new_vec_csv_data) {
        println!("Error: {}", e);
    }
}
