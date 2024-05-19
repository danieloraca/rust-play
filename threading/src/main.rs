use csv::Writer;
use names::Generator;
use rand::{self, Rng};
use std::sync::{mpsc, Arc};
use std::thread;
use uuid::Uuid;

struct CsvRow {
    id: Uuid,
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let output_file = "output.csv";
    let mut writer = Writer::from_path(output_file).expect("Cannot create csv writer");

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);

    let num_threads = 4;
    let max_rows = 1_000_000;
    let rows_per_thread = max_rows / num_threads;
    let batch_size = 100_000;

    for _ in 0..num_threads {
        let tx = Arc::clone(&tx);
        thread::spawn(move || {
            let mut rows = Vec::with_capacity(rows_per_thread);
            let mut rng = rand::thread_rng();
            let mut generator: Generator = Generator::default();

            for _ in 0..rows_per_thread {
                let id = Uuid::new_v4();
                let name: String = generator.next().unwrap().replace(" ", "_");
                let age = rng.gen_range(18..=99) as u8;
                let email = format!("{}@example.com", name.to_lowercase());

                let person = CsvRow {
                    id,
                    name,
                    age,
                    email,
                };
                rows.push(person);

                // Send in batches
                if rows.len() >= batch_size {
                    tx.send(rows).expect("Could not send data!");
                    rows = Vec::with_capacity(batch_size); // Start a new batch
                }
            }

            // Send remaining rows
            if !rows.is_empty() {
                tx.send(rows).expect("Could not send data!");
            }
        });
    }

    drop(tx); // Close the sending end of the channel

    for received in rx {
        for record in received {
            writer
                .write_record(&[
                    record.id.to_string(),
                    record.name,
                    record.age.to_string(),
                    record.email,
                ])
                .expect("Cannot write record");
        }
    }

    writer.flush().expect("Failed to flush the writer");
}
