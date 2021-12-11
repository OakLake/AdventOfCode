// Advent Of Code - 2021 - Day 1
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    // Read input
    let depths: Vec<i16> = load_from_file("./day_1_input.txt");

    part_one(depths.clone());
    part_two(depths.clone());

    Ok(())
}

fn part_one(depths: Vec<i16>) {

    let number_depth_increases: usize = depths
        .windows(2)
        .into_iter()
        .map(|s| &s[1] - &s[0])
        .filter(|f| f > &0)
        .collect::<Vec<i16>>()
        .len();

    println!(
        "Part 1 :: Number of depth increases: {}",
        number_depth_increases
    );
}

fn part_two(depths: Vec<i16>) {
    let number_depth_increases: usize = depths
        .windows(3)
        .into_iter()
        .map(|s| s.iter().sum())
        .collect::<Vec<i16>>()
        .windows(2)
        .into_iter()
        .map(|s| &s[1] - &s[0])
        .filter(|f| f > &0)
        .collect::<Vec<i16>>()
        .len();

    println!(
        "Part 2 :: Number of depth increases: {}",
        number_depth_increases
    );
}

fn load_from_file(file_path: &str) -> Vec<i16> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i16> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i16>().unwrap())
        .collect();

    numbers
}
