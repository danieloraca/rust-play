mod csv_reader;
mod csv_writer;
mod json_reader;
mod processor;
mod types;
use std::time::Instant;

fn main() -> () {
    let start = Instant::now();
    let json_file_name = "raw.json";
    let csv_file_name = "sync_queue.csv";

    // Measure the time taken to read the JSON file
    let json_start = Instant::now();
    let attendances = json_reader::read_json_file(json_file_name);
    let json_duration = json_start.elapsed();
    println!(
        "Time to read JSON file: {} microseconds",
        json_duration.as_micros()
    );

    // Measure the time taken to read the CSV file
    let csv_start = Instant::now();
    let csv_content = csv_reader::read_csv_file(csv_file_name);
    let csv_duration = csv_start.elapsed();
    println!(
        "Time to read CSV file: {} microseconds",
        csv_duration.as_micros()
    );

    // Measure the time taken to write the CSV file
    let write_start = Instant::now();
    csv_writer::write_to_csv(&attendances, &csv_content);
    let write_duration = write_start.elapsed();
    println!(
        "Time to write CSV file: {} microseconds",
        write_duration.as_micros()
    );

    let total_elapsed = start.elapsed();
    println!(
        "Total elapsed time: {} microseconds",
        total_elapsed.as_micros()
    );
}
