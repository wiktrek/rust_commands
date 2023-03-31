use std::{path::Path};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use webbrowser;
#[derive(Serialize, Deserialize, Debug)]
struct Links {
    name: String,
    link: String,
}

pub fn web(link: String) {
    let path = Path::new("src/configs/web.json");
    let file = fs::File::open(path).expect("failed to open");

    let read_to_json:Vec<Links> = serde_json::from_reader(&file).expect("error while reading or parsing");
    println!("{:?}", read_to_json);
    for l in read_to_json {
        if link == l.name {
            let result = webbrowser::open(l.link.as_str());
        println!("{:?}", result);
        }
        println!("{:?}", l.link);
    }
}