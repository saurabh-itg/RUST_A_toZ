use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin().read_line(& mut input).expect("Failed to read the line");
    println!("Hello {} , Have  NICE DAY , I am sur eyou will rock !! :)",input);
}
