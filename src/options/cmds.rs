use std::{path::Path};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Debug)]
struct Cmds {
    name: String,
    cmd: String,
    arg: String,
}
pub fn cmds() {
    println!("Enter name of your cmd from your config");
    let mut option = String::new();
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let cmd = option.replace("\r\n", "");
    cmd_run(cmd)
}
fn check_bash() -> String{
    println!("bash or cmd");
    let mut option = String::new();
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
if option != "bash" || option != "cmd" {
    return "err".to_string();
} else {
return option.to_string()
}

}
fn cmd_run(cmd: String) {
    let path = Path::new("src/configs/cmds.json");
    let file = fs::File::open(path).expect("failed to open");

    let read_to_json:Vec<Cmds> = serde_json::from_reader(&file).expect("error while reading or parsing");
    for cmd_option in read_to_json{
     if cmd != cmd_option.name {
return println!("error: Couldn't find your command")
     }
        let a = check_bash();
        if a == "err" {
            return println!("error")
    }
    let output = Command::new(cmd_option.cmd)
        .arg(cmd_option.arg)
        .output()
        .expect("Failed to execute command");
 assert_eq!(b"cmd result:\n", output.stdout.as_slice());

    }
}