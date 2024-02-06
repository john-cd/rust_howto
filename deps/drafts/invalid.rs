#![allow(dead_code)]
use csv::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
