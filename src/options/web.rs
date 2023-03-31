use std::path::Path;
use std::fs;
fn web() {
    let path = Path::new("src/configs/web.json");
    let file = fs::File::open(path).expect("failed to open");
}