use csv::Writer;
use names::Generator;
use rand::{self, Rng};
use std::sync::{Arc, Mutex};
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
    let writer = Writer::from_path(output_file).expect("Cannot create csv writer");
    let writer = Arc::new(Mutex::new(writer));

    let num_threads = 4;
    let max_rows = 1_000_000;
    let rows_per_thread = max_rows / num_threads;
    let batch_size = 100_000;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let writer = Arc::clone(&writer);
        let handle = thread::spawn(move || {
            // let mut rows = Vec::with_capacity(rows_per_thread);
            let mut rng = rand::thread_rng();
            let mut generator: Generator = Generator::default();
            let mut batch = Vec::with_capacity(batch_size);

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
                batch.push(person);
                if batch.len() >= batch_size {
                    let mut writer = writer.lock().expect("Failed to lock writer");
                    for record in &batch {
                        writer
                            .write_record(&[
                                record.id.to_string(),
                                record.name.clone(),
                                record.age.to_string(),
                                record.email.clone(),
                            ])
                            .expect("Cannot write record");
                    }
                    batch.clear();
                }
            }

            // Send remaining rows
            if !batch.is_empty() {
                let mut writer = writer.lock().expect("Failed to lock writer");
                for record in batch {
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
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    writer
        .lock()
        .expect("Failed to lock writer")
        .flush()
        .expect("Failed to flush the writer");
}
