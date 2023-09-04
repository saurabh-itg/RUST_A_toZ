use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let reversed = reverse_string(&input);
    println!("Reversed string: {}", reversed);
}

fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}