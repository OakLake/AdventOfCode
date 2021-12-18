use std::io::Result;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_4_input.txt");
    match lines_result {
        Ok(mut lines) => {
            let bingo_numbers_line = lines.remove(0);
            let bingo_numbers = bingo_numbers_line.split(',');
            println!("bingo numbers: {:?}", bingo_numbers);
            // for line in lines {
            // }
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
    Ok(())
}
