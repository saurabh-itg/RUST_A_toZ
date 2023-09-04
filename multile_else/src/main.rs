fn main() {
    let integer =4651;
    if integer%2==0{
    println!("NUmber is divisible by 2");}
    else if integer%3==0 {
        println!("Number is divisible by 3");
    }
    else if integer%4==0 {
        println!("Number is divisible by 4");

    }
    else if integer%5 ==0{
        print!("Number is divisible by 5");
    }
    else{
        println!("Number not divisible by 2,3,4 or 5");
    }
}
