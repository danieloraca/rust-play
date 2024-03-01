use csv;
use chrono;
use serde::{Deserialize, Serialize};
use csv_demo::date_convert::deserialize_date;

#[derive(Serialize, Deserialize, Debug)]
//name,location,Date,Rating,Review,Image_Links
struct DataRow {
    name: String,
    location: String,
    #[serde(rename = "Date", deserialize_with = "deserialize_date")]
    date: chrono::NaiveDate,
    #[serde(rename = "Rating")]
    rating: String,
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String,
}

fn main() {

    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    let mut records = csv_reader.deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<DataRow>>();

    records.sort_by_key(|r| r.date);

    records.iter().for_each(|record| {
        println!("{:?}", record);
    });
    println!("Number of rows: {:?}", records.len());
}
