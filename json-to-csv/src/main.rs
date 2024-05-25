mod csv_reader;
mod csv_writer;
mod json_reader;
mod processor;
mod types;

fn main() -> () {
    let start = std::time::Instant::now();
    let json_file_name = "raw.json";
    let csv_file_name = "sync_queue.csv";
    let attendances = json_reader::read_json_file(json_file_name);
    let csv_content = csv_reader::read_csv_file(csv_file_name);

    csv_writer::write_to_csv(&attendances, &csv_content);

    let elapsed = start.elapsed();
    println!("Elapsed time: {} microseconds", elapsed.as_micros());
}
