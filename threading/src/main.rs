use std::{thread::{self, JoinHandle}, time::Duration};
use csv::Writer;
use micro_rand::*;
use std::sync::mpsc::{channel, Sender, Receiver};
use uuid::Uuid;
use names::Generator;

fn main() {
    let output_file = "output.csv";
    let mut writer = Writer::from_path(output_file).expect("Cannot create csv writer");
    let mut rows: Vec<&str> = vec![];

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    let thread_fn1 = move || {
        let mut rows = vec![];
        let mut r = Random::new(123475765765);
        let rnd = r.next_int_i32(0, 20000);
            let mut generator: Generator = Generator::default();
        for i in 0..100000000 {
            let id = Uuid::new_v4();
            let name: String = generator.next().unwrap();
            let age = 10;
            let email = format!("{}@gmail.com", name.to_lowercase());
            rows.push(format!("{},{},{},{},{}", i+1, id, name, age, email));
        };

        for val in rows {
            tx.send(val).unwrap();
        }
    };

    thread::spawn(thread_fn1);

    for received in rx {
        writer.write_record(&[received]).expect("Cannot write record");
    }

}

