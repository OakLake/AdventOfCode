use std::io::Result;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_6_input.txt");
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
    let mut state: Vec<i8> = lines[0]
        .split(",")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    // Simulate
    let num_days: i16 = 256;
    // let num_fish  = sim1(num_days, state);  // 383160
    let num_fish  = sim2(num_days, state);  // 1721148811504

    

    println!("Number of fish after {} days: {}", num_days, num_fish);
}

fn step_fish(days: i8) -> i8 {
    if days == 0 {
        return 6;
    } else {
        return days - 1;
    }
}

fn sim1(num_days: i16, state:Vec<i8>) -> usize{
    let mut new_fish_count = 0;
    let mut state_ = state.clone();
    for day in 0..num_days {
        println!("Day: {}", day);
        state_.iter_mut().for_each(|d| *d = step_fish(*d));
        state_.resize(state_.len() + new_fish_count, 8);
        new_fish_count = state_.iter().filter(|d| **d == 0).count();
    }

    state_.len()
}

fn sim2(num_days: i16, state:Vec<i8>) -> u64{
    let mut state_: [u64; 9] = [0; 9];
    for i in 0..9 {
        state_[i] = state.iter().filter(|v| **v == i as i8).count() as u64;
    }

    for day in 0..num_days {
        println!("Day: {}", day + 1);

        let zeroth = state_[0];

        state_.rotate_left(1);
        state_[6] += zeroth;

    }

    state_.iter().map(|v| *v).sum::<u64>()
}
