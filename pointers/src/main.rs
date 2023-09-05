fn main() {
    let x = 5;
    let y = &x; // Immutable reference to x
    println!("Value of x: {}", x);
    let mut a = 5;
    let b = &mut a; // Mutable reference to a
    *b += 1; // Modify the value through the reference
    println!("Value of a: {}", a);
    let c = 42;
    let raw_ptr: *const i32 = &c;//Raw pointer
    let value = unsafe { *raw_ptr };
    println!("Value from raw pointer: {}", value);
    let d = Box::new(42);//Smart pointer 
    println!("Value from Box: {}", *d);
}
