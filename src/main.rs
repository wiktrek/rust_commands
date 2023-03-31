use std::io;
// 
mod options;
fn main() {
    let mut option = String::new();
    println!("Enter option");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let replace_option = option.replace("\r\n", "");
    match replace_option.as_str() {
    "web" => web(),
    _ => panic!("Invalid option")
    }
}
fn web() {
    let mut option = String::new();
    println!("Enter the name from your config (configs/web.json)");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    let replace_option = option.replace("\r\n", "");
options::web(replace_option)
}