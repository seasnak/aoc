use std::fs;
use regex::Regex;

const INPUT_FILE: &str = "input/y24/d03.txt";

fn part_one() -> std::io::Result<i32> {
    let re = Regex::new(r"mul\((?<a>)[0-9]{0,3},(?<b>)[0-9]{0,3}\)").unwrap();

    let binding = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    //let mults = re.captures(&binding).map_or("", |m| &(m["a"].parse::<i32>().unwrap() * m["b"].parse::<i32>().unwrap()).to_string());
    let mults_str = re.captures(&binding);
    let mults = mults_str.iter();


    let mut tot_sum: i32 = 0;
    //for mult in mults.split(' ') {
        //tot_sum += mult.parse().unwrap();
    //}
    for mult in mults {
        let val_a = mult["a"].parse::<i32>().unwrap();
        let val_b = mult["b"].parse::<i32>().unwrap();
        tot_sum += val_a * val_b;
    }
    
    Ok(0)
}

fn part_two() -> std::io::Result<u32> {

    Ok(0)
}

pub fn solve(part:i32) {
    println!("=== PART {} ===", part);
    match part {
        1 => println!("{}", part_one().unwrap()),
        2 => println!("{}", part_two().unwrap()),
        _ => panic!("Unknown Part")
    }
}
