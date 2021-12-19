use std::io::Result;
// use itertools::Itertools;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_4_input.txt");
    match lines_result {
        Ok(lines) => {
            part_one(lines);
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
    Ok(())
}

fn part_one(lines: Vec<String>) {
    let mut mut_lines = lines.clone();
    let bingo_numbers_line = mut_lines.remove(0);
    let bingo_numbers: Vec<i16> = bingo_numbers_line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("bingo numbers: {:?}", bingo_numbers);

    let mut boards = create_boards(mut_lines);

    let mut winner: i16 = -1;
    for bingo in bingo_numbers {
        if winner != -1 {
            break;
        }
        for board_ix in 0..boards.len() {
            for cluster_ix in 0..boards[board_ix].len() {
                boards[board_ix][cluster_ix].retain(|i| *i != bingo);

                if boards[board_ix][cluster_ix].len() == 0 {
                    println!("Winner is board #{}", board_ix);
                    winner = board_ix as i16;
                    let sum: i16 = boards[board_ix][0..5]
                        .iter()
                        .map(|b| b.iter().sum::<i16>())
                        .into_iter()
                        .sum();
                    println!("Results = {}", sum * bingo);
                }
            }
        }
    }
}

fn create_boards(lines: Vec<String>) -> Vec<Vec<Vec<i16>>> {
    let mut boards: Vec<Vec<Vec<i16>>> = Vec::new();
    let mut board: Vec<Vec<i16>> = vec![vec![0; 5]; 10];
    let mut arr_ix: usize = 0;
    let mut mut_lines = lines.clone();
    // Create the boards data structure
    // Remove first empty line
    mut_lines.remove(0);
    for line in mut_lines {
        println!("{}", line);
        if line != "" {
            let v: Vec<i16> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for (ix, val) in v.iter().enumerate() {
                // 5: magic!
                board[5 + ix][arr_ix] = val.clone()
            }
            board[arr_ix] = v.clone();

            arr_ix += 1;
        } else {
            arr_ix = 0;
            println!("board: {:?}", board);
            println!("##############");
            boards.push(board.clone());
        }
    }
    println!("board: {:?}", board);
    println!("##############");
    // Push the board at the end of the file, that doesn't have an empty line after it
    boards.push(board.clone());

    boards
}
