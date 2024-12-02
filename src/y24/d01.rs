use std::io;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;

const DAY: &str = "01";
const INPUT_FILE: &str = "input/y24/d01.txt";

fn part_one(reader:BufReader<File>) -> Result<i32,io::Error> {
    let answer = 0;
    let content = reader.lines().flatten();
    println!("{}", answer);
    Ok(answer)

}

fn part_two(reader:BufReader<File>) -> Result<i32,io::Error> {
    let answer = 0;
    let content = reader.lines().flatten();
    Ok(answer)
}

fn solve(part:i32=1) {
    if part == 1 {
        println!("=== PART 1 ===");
        let f = File::open(INPUT_FILE);
        println!("{:?}", part_one(BufReader::new(f)));
    }
    else {
        println!("=== PART 2 ===");
        let f = File::open(INPUT_FILE);
        println!("{:?}", part_two(BufReader::new(f)));
    }
    
}
