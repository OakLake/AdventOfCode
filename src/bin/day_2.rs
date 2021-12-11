// Advent Of Code - 2021 - Day 2
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let instructions: Vec<String> = load_from_file("./day_2_input.txt");

    let mut horz_pos = 0;
    let mut depth = 0;

    for instruction in instructions {
        println!("{}", instruction);
        let instruction: Vec<&str> = instruction.split(" ").collect();
        let command = instruction[0];
        let value = instruction[1].parse::<i32>().unwrap();
        match command {
            "forward" => horz_pos = horz_pos + value,
            "up" => depth = depth - value,
            "down" => depth = depth + value,
            _ => println!("FALSE"),
        }
    }

    println!(
        "Horizontal position: {}, Depth: {}, Results {}",
        horz_pos,
        depth,
        horz_pos * depth
    );

    Ok(())
}

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let instructions: Vec<String> = reader.lines().map(|s| s.unwrap().to_string()).collect();

    instructions
}