use std::cmp;
use std::io::Result;

use regex::Regex;

mod helpers;

fn main() -> Result<()> {
    let lines_result = helpers::load_from_file("./input/day_5_input.txt");
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
    let mut xy_coords: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut number_intersections = 0;

    let re = Regex::new(r"[0-9]+").unwrap();

    for l in lines {
        let coords: Vec<i32> = re
            .captures_iter(&l)
            .into_iter()
            .map(|cap| cap[0].parse().unwrap())
            .collect();

        let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);

        let is_vertical_or_horizontal = (x1 == x2) || (y1 == y2);
        println!("Processing: {}", l);
        if is_vertical_or_horizontal {
            let x_start = cmp::min(x1, x2);
            let x_end = cmp::max(x1, x2) + 1;
            let y_start = cmp::min(y1, y2);
            let y_end = cmp::max(y1, y2) + 1;

            for x in x_start..x_end {
                for y in y_start..y_end {
                    println!("{},{} ", x, y);
                    xy_coords[x as usize][y as usize] += 1;
                }
            }
        } else {
            for (x, y) in get_xy_coords(x1, x2, y1, y2) {
                println!("{},{} ", x, y);
                xy_coords[x as usize][y as usize] += 1;
            }
        }
    }

    for ix in 0..1000 {
        for iy in 0..1000 {
            if xy_coords[ix as usize][iy as usize] > 1 {
                number_intersections += 1;
            }
        }
    }
    println!("Intersections: {}", number_intersections);
}

fn get_xy_coords(x1: i32, x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    let unit_x = x2 - x1;
    let unit_y = y2 - y1;

    let x_range: Vec<i32>;
    let y_range: Vec<i32>;

    if unit_x < 0 {
        x_range = (x2..x1 + 1).rev().collect();
    } else {
        x_range = (x1..x2 + 1).collect();
    }

    if unit_y < 0 {
        y_range = (y2..y1 + 1).rev().collect();
    } else {
        y_range = (y1..y2 + 1).collect();
    }
    let coords: Vec<(i32, i32)> = x_range
        .clone()
        .iter()
        .zip(y_range.clone().iter())
        .map(|(x, y)| (*x, *y))
        .collect();

    coords
}
