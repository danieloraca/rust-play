use csv:: {Writer};
use uuid::Uuid;
use names::Generator;
use micro_rand::*;

struct CsvRow<'a> {
    id: Uuid,
    name: &'a String,
    age: u8,
    email: String,
}

fn main() {
    let mut generator: Generator = Generator::default();
    let mut rnd = Random::new(1234);

    let path: &str = "person.csv";
    let max_names: usize = 1_000_000;
    let mut csv_writer = Writer::from_path(path).unwrap();
    for _ in 0..max_names {
        let name: String = generator.next().unwrap();
        let person: CsvRow = CsvRow {
            id: Uuid::new_v4(),
            name: &name,
            age: rnd.next_int_i32(18, 99) as u8,
            email: format!("{}@example.com", name.to_lowercase()),
        };
        csv_writer.write_record([
            &person.id.to_string(),
            &person.name,
            &person.age.to_string(),
            &person.email,
        ]).unwrap();
    }

    csv_writer.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv() {
        let path: &str = "person.csv";
        let mut csv_reader = Reader::from_path(path).unwrap();
        for result in csv_reader.records() {
            let record = result.unwrap();
            println!("{:?}", record);
        }

    }
}

