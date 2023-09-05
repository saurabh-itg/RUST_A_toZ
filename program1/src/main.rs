use std::io;
fn main() {
    let mut num =String::new();

    println!("Input a number :");
    io::stdin().read_line(& mut num).expect("Failed to read line");
    let n :i32 = num.trim().parse().expect("Not a valid number");

    let mut sq = n*n;
    println!("Square of number is {}",sq);

}
