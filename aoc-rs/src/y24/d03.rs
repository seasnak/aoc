use std::fs;
use regex::Regex;

const INPUT_FILE: &str = "input/y24/d03.txt";

fn part_one() -> std::io::Result<u32> {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();

    let binding = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    
    let mult_capts = re.captures_iter(&binding.trim()); // regex to capture all strings mul(a,b)
    let mults = mult_capts.map(|m| m["a"].parse::<i32>().unwrap() * m["b"].parse::<i32>().unwrap());
    let tot_sum: i32 = mults.sum();
    
    Ok(tot_sum as u32)
}

fn part_two() -> std::io::Result<u32> {
    let binding = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");

    let re_inactive = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let re_inactive_end = Regex::new(r"don't\(\).*").unwrap();
    let re_mults = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap(); 
    let re_nl = Regex::new(r"\n").unwrap();

    let binding_compact = re_nl.replace_all(&binding, "");
    let binding_no_inactive = re_inactive.replace_all(&binding_compact, "");
    let binding_clean = re_inactive_end.replace(&binding_no_inactive, "");

    let active_capts = re_mults.captures_iter(&binding_clean); // regex to capture all strings mul(a,b) 
    let mults = active_capts.map(|m| m["a"].parse::<i32>().unwrap() * m["b"].parse::<i32>().unwrap());
    let tot_sum: i32 = mults.sum();

    Ok(tot_sum as u32)
}

pub fn solve(part:i32) {
    println!("=== PART {} ===", part);
    match part {
        1 => println!("{}", part_one().unwrap()),
        2 => println!("{}", part_two().unwrap()),
        _ => panic!("Unknown Part")
    }
}
