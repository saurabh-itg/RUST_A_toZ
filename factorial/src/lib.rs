use std::io;

fn main() {
    println!("Enter the number whose factorial you want to calculate:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Failed to read line");

    factorial(a);
}

fn factorial(a: i32) {
    let mut counter: i32 = a - 1;
    let mut f: i32 = a;
    loop {
        f = f * counter;
        counter -= 1;
        if counter == 1 {
            break;
        }
    }
    println!("Factorial of {a} is {f}");
}
