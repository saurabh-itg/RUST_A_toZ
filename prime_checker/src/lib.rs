use std::io;
fn main() {
    println!("enter the number :");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the number:");
    let num :i32 = input.trim().parse().expect("Not a valid number:");
    
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}