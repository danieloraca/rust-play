use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct MyData {
    #[serde(rename = "__schema")]
    schema: Schema,
}

#[derive(Debug, Deserialize)]
struct Schema {
    #[serde(rename = "queryType")]
    query_type: QueryType,
    types: Vec<Type>,
}

#[derive(Debug, Deserialize)]
struct QueryType {
    #[serde(rename = "name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Type {
    #[serde(rename = "name")]
    schema_type_name: String,
    #[serde(rename = "fields")]
    schema_type_fields: Option<Vec<SchemaTypeField>>,
}

#[derive(Debug, Deserialize)]
struct SchemaTypeField {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "type")]
    field_type: FieldType,
}

#[derive(Debug, Deserialize)]
struct FieldType {
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "ofType")]
    of_type: Option<Box<FieldType>>,
}

fn main() {
    let mut file = File::open("temp3.json").expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let parsed_data: MyData = serde_json::from_str(&contents).expect("Failed to parse JSON");

    println!("QUERY TYPE {:?}", parsed_data.schema.query_type.name);

    for t in parsed_data.schema.types {
        println!("TYPE {:?}", t.schema_type_name);
        if let Some(schema_type_fields) = t.schema_type_fields {
            for f in schema_type_fields {
                println!("FIELD {:?}", f.name);
                match f.field_type.name {
                    Some(name) => println!("FIELD TYPE {:?}", name),
                    None => (),
                }
                match f.field_type.of_type {
                    Some(of_type) => match of_type.name {
                        Some(name) => println!("OF TYPE {:?}", name),
                        None => (),
                    },
                    None => (),
                }
            }
        }
    }
}
