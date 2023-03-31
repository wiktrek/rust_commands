use std::io;
use webbrowser;
fn main() {
    let mut option = String::new();
    println!("Enter option");
    io::stdin()
    .read_line(&mut option).expect("Couldn’t read from stdin");
    println!("Hello, world!");
    github();
println!("fortnite")
}
fn github() -> Result<(), std::io::Error> {
    webbrowser::open("http://github.com")
}   