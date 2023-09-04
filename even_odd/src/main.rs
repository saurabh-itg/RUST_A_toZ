use std::io;
fn main() {
    println!("Program to check even or odd");
    println!("ENter your number :");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let a :i32 =input.trim().parse().expect("enter a valid number");
    iseven(a);
}
fn iseven(a:i32){
    if a%2==0{
        println!("{a} is a even number ");
    }
    else{
        println!("{a} is odd number");
    }

}