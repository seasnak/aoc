//use std::io;
//use std::env;
//use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = "input/y24/d01.txt";

fn part_one(reader:BufReader<File>) -> std::io::Result<uszie> {
    let answer = 0;
    let content = reader.lines().flatten();
        
    Ok(answer)

}

//fn part_two(reader:BufReader<File>) -> Result<usize> {
//    let answer = 0;
//    let content = reader.lines().flatten();
//    Ok(answer)
//}

pub fn solve(part:i32=1) -> std::io::Result<()> {
    if part == 1 {
        println!("=== PART 1 ===");
        let f = File::open(INPUT_FILE)?;
        let mut buf = BufReader::new(f);
        println!("{}", part_one(buf));
    }
    else {
        //println!("=== PART 2 ===");
        //let f = File::open(INPUT_FILE)?;
        //println!("{}", part_two(BufReader::new(f)));
    }

    Ok(())    
}

//fn main() {
//    let args: Vec<String> = env::args().collect();
//    assert_eq!(args.len(), 2);
//
//    solve(args[1].as_str().parse().unwrap());
//
//}
