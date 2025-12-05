use std::fs;

const INPUT_FILE: &str = "input/y24/d01.txt";

fn part_one() -> std::io::Result<u32> {
    
    let binding = fs::read_to_string(INPUT_FILE).expect("File Read Error");
    let content = binding.trim();
    let conts = content.split("\n").collect::<Vec<&str>>();

    let (mut a, mut b) = (Vec::<i32>::with_capacity(conts.len()), Vec::<i32>::with_capacity(conts.len()));
    for line in conts {
        let pos = line.chars().position(|x| x == ' ').unwrap();
        a.push((&line[0..pos]).parse().unwrap());
        b.push((&line[pos..].trim()).parse().unwrap());
    }

    a.sort();
    b.sort();
    //dbg!("{}, {}", a, b);
    
    let out = a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<u32>();
    Ok(out)
}

fn part_two() -> std::io::Result<usize> {
    
    let binding = fs::read_to_string(INPUT_FILE).expect("File Read Error"); 
    let content = binding.trim();
    let conts = content.split("\n").collect::<Vec<&str>>();
    
    //let (mut a, mut b) = (Vec::<i32>::with_capacity(conts.len()), Vec::<i32>::with_capacity(conts.len()));
    //for line in conts {
    //    let pos = line.chars().position(|x| x == ' ').unwrap();
    //    a.push((&line[0..pos]).parse().unwrap());
    //    b.push((&line[pos..]).parse().unwrap());
    //}
    
    let mut a = Vec::<usize>::with_capacity(conts.len());
    let mut b = [0; 100000]; // all values are 5 digits
    
    for line in conts {
        let pos = line.chars().position(|x| x == ' ').unwrap();
        a.push((&line[0..pos]).parse().unwrap());
        b[line[pos..].trim().parse::<usize>().unwrap()] += 1;
    }

    let out = a.into_iter().map(|i| b[i] * i).sum::<usize>();
    Ok(out)
}

pub fn solve(part:i32) {
    println!("=== PART {} ===", part);
    match part {
        1 => println!("{}", part_one().unwrap()),
        2 => println!("{}", part_two().unwrap()),
        _ => panic!("Unknown Part")
    }
}
