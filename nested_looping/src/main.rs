fn main() {
    let mut count =0;

    'countup : loop {
        println!("count is {count}");
        let mut tick=10;

        println!("tick value is {tick}");
        loop{
            if tick==5{
                break ;
            }
            if count ==4{
                break 'countup;

            }
            tick-=1;

        }
        count+=1;
    }
    println!("final count is {count}");
}