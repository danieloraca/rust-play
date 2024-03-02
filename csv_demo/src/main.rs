use csv;
use chrono;
use serde::{Deserialize, Serialize};
use csv_demo::parse_image_links::{deserialize_image_links, serialize_image_links};
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
    #[serde(rename = "Image_Links", deserialize_with = "deserialize_image_links", serialize_with = "serialize_image_links")]
    image_links: Option<Vec<url::Url>>,
}

fn read_reviews_csv() -> Vec<DataRow> {
    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    csv_reader.deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<DataRow>>()
}

fn write_reviews_csv(records: Vec<DataRow>) {
    let mut csv_writer = csv::Writer::from_path("reviews_data_clean.csv").unwrap();

    records
        .iter()
        .try_for_each(|record| csv_writer.serialize(record))
        .unwrap();
}

fn main() {
    let mut records = read_reviews_csv();
    records
        .sort_by_key(|r| r.date);
    write_reviews_csv(records);

    // let json = serde_json::ser::to_string_pretty(&records).unwrap();
    //
    // println!("{}", json);

    // let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    // let mut records = csv_reader.deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<DataRow>>();
    //
    // records.sort_by_key(|r| r.date);
    // records
    //     .iter()
    //     .take(100)
    //     .for_each(|record| record.image_links.iter().for_each(|link| link.iter().for_each(|l| println!("{}", l))));
    // println!("Number of rows: {:?}", records.len());
}
