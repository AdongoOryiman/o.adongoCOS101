use std::fs;

fn main() {
    fs::remove_file("C:/Users/owner/OneDrive/Documents/PAU/o.adongoCOS101/week-8/practice_3/src/data.txt").expect("could nit remove file");
    println!("file is removed");
}
