use std::io::Result;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_4_input.txt");
    match lines_result {
        Ok(lines) => {
            part_one_and_two(lines);
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
    Ok(())
}

fn part_one_and_two(lines: Vec<String>) {
    let mut mut_lines = lines.clone();
    let bingo_numbers_line = mut_lines.remove(0);
    let bingo_numbers: Vec<i32> = bingo_numbers_line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("bingo numbers: {:?}", bingo_numbers);

    let mut boards = create_boards(mut_lines);

    let mut winners: Vec<[i32; 2]> = vec![];
    for bingo in bingo_numbers {
        for board_ix in 0..boards.len() {
            for cluster_ix in 0..boards[board_ix].len() {
                boards[board_ix][cluster_ix].retain(|i| *i != bingo);

                if boards[board_ix][cluster_ix].len() == 0 {
                    let winner = board_ix as i32;
                    let sum: i32 = boards[board_ix][0..5]
                        .iter()
                        .map(|b| b.iter().sum::<i32>())
                        .into_iter()
                        .sum();
                    let winner_and_results: [i32; 2] = [winner, sum * bingo];
                    if !winners.clone().into_iter().any(|[w, _s]| w == winner) {
                        winners.push(winner_and_results);
                    }
                }
            }
        }
    }
    for [board_ix, result] in winners.clone() {
        println!("Winner board #{}, result: {}", board_ix, result)
    }
}

fn create_boards(lines: Vec<String>) -> Vec<Vec<Vec<i32>>> {
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut board: Vec<Vec<i32>> = vec![vec![0; 5]; 10];
    let mut arr_ix: usize = 0;
    let mut mut_lines = lines.clone();
    // Create the boards data structure
    // Remove first empty line
    mut_lines.remove(0);
    for line in mut_lines {
        println!("{}", line);
        if line != "" {
            let v: Vec<i32> = line
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
