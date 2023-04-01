use std::{path::Path};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Debug)]
struct Cmds {
    name: String,
    cmd: String,
    os: String,
}
pub fn cmds() {
    println!("Enter name of your cmd from your config");
    let mut option = String::new();
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let cmd = option.replace("\r\n", "");
    cmd_run(cmd)
}
fn check_bash(cmd: Cmds){
    println!("bash or cmd");
    let mut option = String::new();
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let replace_option = option.replace("\r\n", "");
    println!("{}", replace_option);
if replace_option == "bash"{
    if replace_option == cmd.os.as_str() || cmd.os.as_str() == "both"{
    return bash(cmd);

    }
        return println!("error")

} else if replace_option == "cmd" {
    if replace_option == cmd.os.as_str() || cmd.os.as_str() == "both"{

    return cmd_fn(cmd)
    
    }
    return println!("error: wrong option")
}else {

return println!("error: wrong option (bash/cmd)")
}

}
fn cmd_run(cmd: String) {
    let path = Path::new("src/configs/cmds.json");
    let file = fs::File::open(path).expect("failed to open");

    let read_to_json:Vec<Cmds> = serde_json::from_reader(&file).expect("error while reading or parsing");
    for cmd_option in read_to_json{
     if cmd == cmd_option.name {
    return check_bash(cmd_option);
     }
} 
return println!("error: Couldn't find your command")
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