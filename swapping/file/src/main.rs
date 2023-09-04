fn main() {
    let mut a = 10;
    let mut b = 20;

    println!("Before swapping: a = {}, b = {}", a, b);

    // Swap the values using a temporary variable
    let temp = a;
    a = b;
    b = temp;

    println!("After swapping: a = {}, b = {}", a, b);
}