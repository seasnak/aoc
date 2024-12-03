use y24::*;
use std::env;

mod y24;

const YEAR:i32 = 2024; 

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    let part: i32 = args[2].parse::<i32>().unwrap();
    //dbg!{"day:{}, part:{}", day_num, part};

    match day_num {
        1 => d01::solve(part),
        2 => d02::solve(part),
        _ => panic!("Unknown day {}", day_num),
    }

}
