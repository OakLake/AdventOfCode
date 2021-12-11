// Advent Of Code - 2021 - Day 1
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    // Read input
    let depths: Vec<i16> = load_from_file("./input.txt");

    let mut depths_shifted: Vec<i16> = depths.clone();
    // Shift vector by one and maintain correct values
    depths_shifted.insert(0, depths[0]);
    depths_shifted.pop();

    // Compute element-wise difference between shifted vectors and keep positive values.
    let depth_changes: Vec<i16> = depths
        .iter()
        .zip(depths_shifted.iter())
        .map(|(&b, &v)| b - v)
        .filter(|v| v > &0)
        .collect();

    println!("Number of depth increases: {}", depth_changes.len());

    Ok(())
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
