use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

pub fn load_from_file(file_path: &str) -> Result<Vec<String>> {
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
