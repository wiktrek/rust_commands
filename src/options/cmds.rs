use std::{path::Path};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Debug)]
struct Cmds {
    name: String,
    cmd: String,
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
    let replace_option = option.replace("\r\n", "");
    println!("{}", replace_option);
if replace_option == "bash" || replace_option == "cmd" {
    
    return option.to_string()
} else {
return "err".to_string()
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
       let a_str = a.as_str(); 
        if a_str == "err" {
            return println!("error: wrong option")
    };
if a_str == "bash" {
    print!("running bash command");
    bash(cmd_option)
} else if a_str == "cmd" {
println!("running cmd command");
cmd_fn(cmd_option)
} else {
    println!("error: wrong option")
}
}
}
fn bash(cmd: Cmds) {
      let output = Command::new("sh")
            .arg("-c")
            .arg(cmd.cmd)
            .output()
            .expect("failed to execute process");
        println!("{:?}",output)
}
fn cmd_fn(cmd: Cmds) {
    let output = Command::new("cmd")
            .args(["/C", cmd.cmd.as_str()])
            .output()
            .expect("failed to execute process");
println!("{:?}", output)
}