use std::fs;
use std::collections::HashSet;
use std::cmp::min;

const INPUT_FILE: &str = "input/y24/d02.txt";

fn part_one() -> std::io::Result<u32> {
    let binding = fs::read_to_string(INPUT_FILE).expect("File Read Error");
    let content_vec = binding.trim().split('\n').collect::<Vec<&str>>();
    
    let mut num_safe : u32 = 0;

    for report in content_vec {
        let rep_vec = report.split(' ').collect::<Vec<&str>>();
        
        let rep_set: HashSet<&&str> = HashSet::from_iter(rep_vec.iter());
        if rep_set.len() != rep_vec.len() { continue; } // unsafe report -- duplicate or both 
                                                        // asc/desc

        let is_asc: bool = rep_vec[0].parse::<i32>().unwrap() - rep_vec[1].parse::<i32>().unwrap() > 0;
        for (i, rep) in rep_vec.iter().enumerate() {
            if i + 1 == rep_vec.len() { num_safe += 1; break; } // reached end so safe
            
            let first = rep.parse::<i32>().unwrap();
            let second = rep_vec[i+1].parse::<i32>().unwrap();
            let level_diff = first - second;
            let is_asc_check: bool = level_diff > 0;
            if is_asc_check != is_asc { break; } // unsafe report -- changed direction
            else if level_diff.abs() > 3 { break; } // unsafe report -- change of level > 3
        }

    }
    

    Ok(num_safe)
}

fn part_two() -> std::io::Result<u32> {
    let binding = fs::read_to_string(INPUT_FILE).expect("File Read Error");
    let content_vec = binding.trim().split('\n').collect::<Vec<&str>>();

    let mut num_safe : u32 = 0;

    for report in content_vec {
        let rep_vec = report.split(' ').collect::<Vec<&str>>();

        let rep_set: HashSet<&&str> = HashSet::from_iter(rep_vec.iter());
        if rep_vec.len() - rep_set.len() > 1 { continue; } // unsafe report -- more than 1 duplicate or both asc/desc
                                                           //
        let mut net_change = 0;
        let mut num_asc = 0;
        let mut num_desc = 0;
        let mut has_skipped: bool = false;
        for (i, rep) in rep_vec.iter().enumerate() {
            if i + 1 == rep_vec.len() { // reached end
                if has_skipped && min(num_asc, num_desc) >= 1 { break; }
                if min(num_asc, num_desc) < 2 { num_safe += 1; } 
                break; 
            }
            
            let first = rep.parse::<i32>().unwrap();
            let second = rep_vec[i+1].parse::<i32>().unwrap();
            let level_diff = first - second;
            if level_diff.abs() > 3 {
                if has_skipped { break; }

                has_skipped = true;
                if i + 2 == rep_vec.len() { break; }
                let third = rep_vec[i+2].parse::<i32>().unwrap();
                let skipped_level_diff = first - third;
                if skipped_level_diff.abs() > 3 { break; }
            }

            if level_diff > 0 { net_change += 1; num_asc += 1; }
            else if level_diff < 0 { net_change -= 1; num_desc += 1; }
        }
        println!("{}, {}", rep_vec, num_safe);
    }

    Ok(num_safe)
}

pub fn solve(part:i32) {
    println!("=== PART {} ===", part);
    match part {
        1 => println!("{}", part_one().unwrap()),
        2 => println!("{}", part_two().unwrap()),
        _ => panic!("Unknown Part")
    }
}
