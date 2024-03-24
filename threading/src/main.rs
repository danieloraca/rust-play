use std::{thread::self};
use csv::Writer;
use micro_rand::*;
use std::sync::mpsc::{channel, Sender};
use uuid::Uuid;
use names::Generator;

struct CsvRow {
    id: Uuid,
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let output_file = "output.csv";
    let mut writer = Writer::from_path(output_file).expect("Cannot create csv writer");

    let (tx, rx): (Sender<Vec<CsvRow>>, _) = channel();

    let num_threads = 4;
    for _ in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut rows = Vec::with_capacity(250_000);
            let mut r = Random::new(123475765765);
            //let rnd = r.next_int_i32(0, 20000);
            let mut generator: Generator = Generator::default();
            for _ in 0..250_000 {
                let id = Uuid::new_v4();
                let name: String = generator.next().unwrap();
                let age = r.next_int_i32(18, 99) as u8;
                let email = format!("{}@gmail.com", name.to_lowercase());
                let person = CsvRow { id, name, age, email };

                rows.push(person);
            }
            let _ = tx.send(rows);
        });
    }

    drop(tx);

    for received in rx {
        for record in received {
                        writer.write_record(&[format!("{},{},{},{}", record.id, record.name, record.age, record.email)]).expect("Cannot write record");
        }
    }
}
