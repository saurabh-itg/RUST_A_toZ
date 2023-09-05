use std::io;
fn main() {
    let mut input =String::new();
    println!("Enter a string having commas :");
    io::stdin().read_line(& mut input).expect("Failed to read the line ");
    // Split the string into substrings using a comma as the delimiter
    let parts: Vec<&str> = input.trim().split(',').collect();

    // Iterate through the resulting substrings
    for part in parts {
        println!("Part: {}", part);
    }
}

