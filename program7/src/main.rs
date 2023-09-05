use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a number :");
    io::stdin().read_line(& mut input).expect("Failed to read the line");
    let num :u32 = input.trim().parse().expect("Not a valid number");
    println!("table of entered number is:\n");
    table(num);

}
fn table(num: u32){
    for i in 1..=10{
        let mut res:u32 =num*i;
        println!("{}*{}={}",num,i,res);
    }
}