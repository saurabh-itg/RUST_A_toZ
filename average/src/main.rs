use std::io;
fn main() {
    println!("Program to input an array of values and finding their average");
    println!("enter the size of array :");
    let mut size_of_array = String::new();
    io::stdin().read_line(&mut size_of_array).expect("Failed to read input");
    let s:i32 = size_of_array.trim().parse().expect("invalid number");

    let mut array :Vec<i32> = Vec ::new();

    for i in 0..s{
        println!("{}th input of array is :",i);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let a:i32=input.trim().parse().expect("INvalid number");
        array.push(a);
       
    }
    let average = calculate_average(&array);
    
    println!("Array entered by the user: {:?}", array);
    println!("Average of array elements: {}", average);
}

fn calculate_average(arr: &Vec<i32>) -> f64 {
    let sum: i32 = arr.iter().sum();
    let size = arr.len() as f64;
    sum as f64 / size
}
