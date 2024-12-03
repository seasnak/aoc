use std::fs;

const DAY: &str = "01";
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
    
    let (mut a, mut b) = (Vec::<i32>::with_capacity(conts.len()), Vec::<i32>::with_capacity(conts.len()));
    for line in conts {
        let pos = line.chars().position(|x| x == ' ').unwrap();
        a.push((&line[0..pos]).parse().unwrap());
        b.push((&line[pos..]).parse().unwrap());
    }

    a.sort();
    b.sort();
    
    //let out = a.iter().map(|i| b[i] * i).sum::<usize>();
    Ok(0)
}

pub fn solve(part:i32) {
    if part == 1 {
        println!("=== PART 1 ===");
        let result = part_one();
        let output = match result {
            Ok(out) => out,
            Err(e) => panic!("Error: {e}"),
        };
        println!("{}", output);
        
    }
    else {
        //println!("=== PART 2 ===");
        //let f = File::open(INPUT_FILE)?;
        //println!("{}", part_two(BufReader::new(f)));
    }
}

//fn main() {
//    let args: Vec<String> = env::args().collect();
//    assert_eq!(args.len(), 2);
//
//    solve(args[1].as_str().parse().unwrap());
//
//}
