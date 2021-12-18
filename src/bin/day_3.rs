use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let binaries = load_from_file("./input/day_3_input.txt");

    match binaries {
        Ok(b) => {
            part_one(b.clone());
            part_two(b);
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn part_one(binaries: Vec<String>) {
    let length = binaries.len() as i16;
    let mut ones_counter: [i16; 12] = [0; 12];
    for telemetry in binaries {
        for (ix, digit) in telemetry.chars().enumerate() {
            ones_counter[ix] += digit.to_digit(10).unwrap() as i16;
        }
    }
    let zeros_counter: [i16; 12] = ones_counter.map(|u| length - u);

    let gamma_rate_str: String = ones_counter
        .iter()
        .zip(zeros_counter.iter())
        .map(|(z, o)| (z > o) as i16)
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("");
    let epsilon_rate_str: String = zeros_counter
        .iter()
        .zip(ones_counter.iter())
        .map(|(o, z)| (o > z) as i16)
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("");

    let gamma_rate = isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_str, 2).unwrap();

    println!("Number of binaries {}", length);
    println!("Gamma   Rate {}", gamma_rate);
    println!("Epsilon Rate {}", epsilon_rate);
    println!("Results: {}", gamma_rate * epsilon_rate);
}

fn part_two(binaries: Vec<String>) {
    let o2_rating = get_rating(binaries.clone(), [0, 1, 1]);
    let co2_rating = get_rating(binaries, [1, 0, 0]);
    let result: i32 = o2_rating as i32 * co2_rating as i32;
    println!(
        "O2: {}, CO2: {}, Results: {}",
        o2_rating, co2_rating, result
    );
}

fn find_bit_count(values: Vec<String>) -> Vec<(i16, i16)> {
    let mut ones_counter: [i16; 12] = [0; 12];
    let length = values.len() as i16;
    for telemetry in values.clone() {
        for (ix, digit) in telemetry.chars().enumerate() {
            ones_counter[ix] += digit.to_digit(10).unwrap() as i16;
        }
    }

    let zeros_counter: [i16; 12] = ones_counter.map(|u| length - u);
    zeros_counter
        .into_iter()
        .zip(ones_counter.into_iter())
        .into_iter()
        .collect::<Vec<(i16, i16)>>()
}

fn get_rating(binaries: Vec<String>, preferred_bits: [i16; 3]) -> i16 {
    let mut rating = binaries;

    for i in 0..12 {
        let bit_counts = find_bit_count(rating.clone());
        let bit_to_choose: i16;
        let count_0 = bit_counts[i].0;
        let count_1 = bit_counts[i].1;
        if count_0 > count_1 {
            bit_to_choose = preferred_bits[0];
        } else if count_0 == count_1 {
            bit_to_choose = preferred_bits[2];
        } else {
            bit_to_choose = preferred_bits[1];
        }

        rating = rating
            .into_iter()
            .filter(|b| b.chars().nth(i).unwrap().to_digit(10).unwrap() as i16 == bit_to_choose)
            .collect::<Vec<String>>();

        if rating.len() == 1 {
            break;
        }
    }

    isize::from_str_radix(&rating[0], 2).unwrap() as i16
}

fn load_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let binaries: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
            Ok(binaries)
        }
        Err(e) => {
            println!("Could not open file: {}", file_path);
            Err(e)
        }
    }
}
