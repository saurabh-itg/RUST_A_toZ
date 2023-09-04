use std::io;
fn main() {
    println!("Sum of two numbers");
    println!("Enter the first number");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    //parse input string to integer
    let a: i32 =input.trim().parse().expect("please enter a valid number");
    println!("Enter the second  number");
    let mut input1 =String::new();
    io::stdin().read_line(&mut input1).expect("failed to read line");
    //parse input string to integer
    let b: i32=input1.trim().parse().expect("please enter a valid number");
    add(a,b);
}
fn add (a: i32 ,b: i32){
    let c=  a+b;
    println!("Sum of {a} and {b} is {c}");
}