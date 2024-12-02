use std::io;
use std::env;
use std::fs;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn part_one(reader: io::BufReader<File>) { 
    let answer = reader.lines().flatten().count();
    Ok(answer); 
}

fn main() {
    start_day(DAY);

    println!("=== PART 1 ===");
    part_one(INPUT_FILE);
    

    
}
