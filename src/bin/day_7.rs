#![feature(int_abs_diff)]

use std::io::Result;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_7_input.txt");
    match lines_result {
        Ok(lines) => {
            let positions: Vec<f32> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
            part_one(positions.clone());
            part_two(positions);
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
    Ok(())
}

fn part_one(h_positions: Vec<i32>) {
    let mut hp = h_positions.clone();

    // Find median
    let median_loc = hp.len() / 2;
    hp.sort();
    let m = hp[median_loc];

    // Calculate fuel cost
    let fuel: u32 = h_positions
        .into_iter()
        .map(|p| p.abs_diff(m))
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    println!("Median: {}", m);
    println!("Fuel: {}", fuel);
}

fn part_two(h_positions: Vec<f32>) {
    let hp = h_positions.clone();

    // Find mean
    let mean = hp.iter().map(|i| *i as f32).sum::<f32>() / hp.len() as f32;
    let mean_r = mean.round() as f32;
    let mean_f = mean as u32 as f32;
    // Calculate fuel cost
    for mean in [mean_r, mean_f] {
        let temp = h_positions.clone();
        let fuel: f32 = temp
            .into_iter()
            .map(|p| ((p-mean).abs() as f32 * (((p-mean).abs() + 1.0) as f32 / 2.0)))
            .collect::<Vec<f32>>()
            .iter()
            .sum();
        println!("Mean: {}", mean);
        println!("Fuel: {}", fuel);
    }
}
