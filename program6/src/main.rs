use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let m: u64 = input.trim().parse().expect("Not a valid number");

    println!("Factorial of {} is {}", m, fact(m));
}

fn fact(m: u64) -> u64 {
    if m == 0 {
        1
    } else {
        m * fact(m - 1)
    }
}

