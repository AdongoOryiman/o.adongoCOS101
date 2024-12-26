use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("C:/Users/owner/OneDrive/Documents/PAU/o.adongoCOS101/week-8/practice_2/src/welcome_message").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
