fn main()
{
    let mut a:i32 =1;
    a=3;
    //shadowing and rebinding
    let mut a=a;//3
    a=a+2;

    const B: i32 = 4;
    let b: i32 =6 ;
    println!("Successful")
}
