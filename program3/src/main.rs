//program to count characters in a string 
use std::io ;
fn main() {
    let mut input =String::new();
    println!("Enter the string :");
    io::stdin().read_line(& mut input).expect("Failed to read the line");
    println!("user entered {}",input.trim());
    let no_spaces_string = input.replace(" ", "");
    let mut chars_count = no_spaces_string.trim().chars().count();
    println!("Total characters in user entered string are {}",chars_count)
}

