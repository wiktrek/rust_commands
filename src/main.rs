use std::io;
// 
mod options;
fn main() {
    let mut option = String::new();
    println!("Enter option");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    println!("Hello, world!");
    match option.as_str() {
    "web" => web(),
    _ => panic!("Invalid option")
    }
    options::web(option);
}
fn web() {

}