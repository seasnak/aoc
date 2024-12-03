use std::fs;

const INPUT_FILE: &str = "input/y24/d02.txt";

fn part_one() -> std::io::Result<u32> {
    let binding = fs::read_to_string(INPUT_FILE).expect("File Read Error");
    let content = binding.trim();
    let content_vec = binding.split('\n').collect::<Vec<&str>>();
    
    let num_safe = 0;

    for line in content_vec {
        let l_vec = line.split(' ').collect::<Vec<&str>>();
        for mut iter in l_vec.iter().enumerate() {
            dbg!("{}", iter.0);
        }
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
