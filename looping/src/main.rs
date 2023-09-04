fn main(){
    let mut counter =0;
    let result =loop{
        counter+=1;
        if counter==30{
        break counter*3;
        }
    };
    println!("the result is {}",result);
}