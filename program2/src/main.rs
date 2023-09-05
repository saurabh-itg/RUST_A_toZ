use std::io;
fn main() {
    let mut input = String::new();
    println!("ENter the first number :");
    io::stdin().read_line(& mut input).expect("Failed to read line");
    let m: i32 = input.trim().parse().expect("Not a valid number:");

    let mut input1 =String::new();
    println!("ENter second number :");
    io::stdin().read_line(& mut input1).expect("Failed to read line");
    let n:i32 =input1.trim().parse().expect("Not a valid number: ");

    let mul :i32 = m*n ;

    println!("{m} * {n} is {}",mul);

}
