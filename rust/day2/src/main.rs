use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Direction {
    Inc,
    Dec,
}

const SAFE_DIFF: i32 = 3;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let mut safe_levels = 0;

    for file in read_to_string("input").unwrap().lines() {
        let level = file
            .split_ascii_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        safe_levels += count_safe_levels(level);
    }

    safe_levels
}

fn part_2() -> i32 {
    let mut safe_levels = 0;

    for file in read_to_string("input").unwrap().lines() {
        let level = file
            .split_ascii_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let level_len = level.len();

        for level in level.into_iter().combinations(level_len - 1) {
            let is_safe = count_safe_levels(level);
            if is_safe == 1 {
                safe_levels += 1;
                break;
            }
        }
    }

    safe_levels
}

fn count_safe_levels(level: Vec<i32>) -> i32 {
    let mut prev_step = None;
    let mut is_safe = true;
    let direction = if level.windows(2).all(|w| w[0] > w[1]) {
        Direction::Dec
    } else {
        Direction::Inc
    };

    for step in level {
        if let Some(prev_step) = prev_step {
            let diff: i32 = prev_step - step;
            if diff.abs() > SAFE_DIFF || diff == 0 {
                is_safe = false;
                break;
            }
            if (diff > 0 && direction != Direction::Dec)
                || (diff < 0 && direction != Direction::Inc)
            {
                is_safe = false;
                break;
            }
        }
        prev_step = Some(step);
    }
    if is_safe {
        1
    } else {
        0
    }
}
