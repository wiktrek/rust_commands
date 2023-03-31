use std::{path::Path};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
use webbrowser;
#[derive(Serialize, Deserialize, Debug)]
struct Links {
    name: String,
    link: String,
}
pub fn web(){
    let mut option = String::new();
    println!("Enter the name from your config (configs/web.json)");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let replace_option = option.replace("\r\n", "");
    if open_link(replace_option).is_ok() {
        println!("\n")
    }    
}

pub fn open_link(link: String) -> Result<(), std::io::Error> {
    let path = Path::new("src/configs/web.json");
    let file = fs::File::open(path).expect("failed to open");

    let read_to_json:Vec<Links> = serde_json::from_reader(&file).expect("error while reading or parsing");
    for l in read_to_json {
        if link == l.name {
            let result = webbrowser::open(l.link.as_str());
            return result
        }
    }
    Ok(())
}