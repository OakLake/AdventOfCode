use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let binaries = load_from_file("./input/day_3_input.txt");

    match binaries {
        Ok(b) => {
            let length = b.len() as i16;
            let mut ones_counter: [i16; 12] = [0; 12];
            for telemetry in b {
                for (ix, digit) in telemetry.chars().enumerate() {
                    ones_counter[ix] = ones_counter[ix] + digit.to_digit(10).unwrap() as i16;
                }
            }
            let zeros_counter: [i16; 12] = ones_counter.clone().map(|u| length - u);

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
        Err(e) => {
            println!("Failed To Read File!: {}", e);
        }
    }

    Ok(())
}

fn load_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let binaries: Vec<String> = reader.lines().map(|s| s.unwrap().to_string()).collect();

    Ok(binaries)
}
