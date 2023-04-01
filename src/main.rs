use std::io;
// 
mod options;
fn main() {
    let mut option = String::new();
    println!("Enter option (web, info,cmds)");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let replace_option = option.replace("\r\n", "");
    match replace_option.as_str() {
    "web" => options::web(),
    "info" => options::info(),
    "cmds" => options::cmds(),
    _ => return println!("Invalid option")
    }
}
