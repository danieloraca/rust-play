use crate::types::CsvData;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

pub fn read_csv_file(file_name: &str) -> Result<Vec<CsvData>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let csv_records: Vec<CsvData> = rdr
        .deserialize()
        .map(|result| result.expect("Failed to parse CSV record"))
        .collect();

    Ok(csv_records)
}
