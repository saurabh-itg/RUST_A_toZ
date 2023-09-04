use std::io;
fn main() {
    println!("Program to find maximum of two numbers:");
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the innput");
    let a:i32 =input.trim().parse().expect("enter a valid number:");
    println!("enter the second number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read the innput");
    let b:i32 =input1.trim().parse().expect("enter a valid number:");
    max(a,b);
}
fn max(a:i32,b:i32){
    if a>b{
        println!("{a} is bigger than {b}");
    }
    else if b>a{
        println!("{b} is bigger than {a}");
    }
    else{
        println!("same numbers are entered");
    }

}