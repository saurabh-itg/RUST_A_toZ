use std::io;
//Power Function: Write a function called power that takes two numbers as parameters: a base and an exponent. Calculate and return the result of raising the base to the power of the exponent. Call this function from the main function and print the result.
fn main() {
    println!("Enter the base of the number :");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a : i32 =input.trim().parse().expect("Not a valid number");
    println!("ENter the exponent of the number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let b: i32 =input1.trim().parse().expect("Not a valid number ");
    power(a,b);

}
fn power(a:i32,b:i32){
    let mut count= b;
    let mut c:i32 =a;
    loop{
        c=c*a;
        count-=1;
        if count==1{
            break;
        }
    }
    println!("a exponent b is {c}");
}