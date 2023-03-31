use std::{path::Path};
use std::fs;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Options {
    name: String,
    description: String,
}

pub fn info() {
    let path = Path::new("src/configs/options.json");
    let file = fs::File::open(path).expect("failed to open");

    let read_to_json:Vec<Options> = serde_json::from_reader(&file).expect("error while reading or parsing");
    for option in read_to_json{
        println!("name: {}, description: {} \n", option.name, option.description)
    }
}