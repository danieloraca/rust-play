use csv::Writer;
use micro_rand::*;
use names::Generator;
use std::fs::File;
use std::sync::{mpsc, Arc, Mutex};
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
    //let mut writer = Writer::from_path(output_file).expect("Cannot create csv writer");
    let file = Mutex::new(File::create(output_file).expect("Cannot create csv file"));

    let writer = Arc::new(file);

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    let num_threads = 6;
    let max_rows = 1_000_000;
    let rows_per_thread = max_rows / num_threads;

    for _ in 0..num_threads {
        let tx = Arc::clone(&tx);
        thread::spawn(move || {
            let mut rows = Vec::with_capacity(rows_per_thread);
            let mut r = Random::new(123475765765);
            let mut generator: Generator = Generator::default();
            for _ in 0..rows_per_thread {
                //let id = Uuid::new_v4();
                let name: String = generator.next().unwrap();
                //let age = r.next_int_i32(18, 99) as u8;
                //let email = format!("{}@gmail.com", name.to_lowercase());
                //let person = CsvRow { id, name, age, email };

                let person = CsvRow {
                    id: Uuid::new_v4(),
                    name,
                    age: r.next_int_i32(18, 99) as u8,
                    email: generator.next().unwrap().to_lowercase() + "@gmail.com",
                };

                let tx = tx.lock().unwrap();
                tx.send(person).expect("Cannot send person");
                //rows.push(person);
            }
            //let _ = tx.send(rows);
        });
    }

    drop(tx);

    //let mut writer = writer.lock().unwrap();
    //let mut csv_writer = Writer::from_writer(&mut *writer);

    let mut csv_writer = {
        let writer = writer.lock().unwrap();
        Writer::from_writer(writer)
    };

    for received in rx {
        //for record in received {
        //                writer.write_record(&[format!("{},{},{},{}", record.id, record.name, record.age, record.email)]).expect("Cannot write record");
        //}

        csv_writer
            .write_record(&[
                received.id.to_string(),
                received.name,
                received.age.to_string(),
                received.email,
            ])
            .expect("Failed to write record");
    }
}
