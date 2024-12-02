use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Direction {
    Inc,
    Dec,
}

const SAFE_DIFF: i32 = 3;

fn main() {
    println!("Part 1: {}", part_1());
}

fn part_1() -> i32 {
    let mut safe_levels = 0;

    for file in read_to_string("input").unwrap().lines() {
        let level = file
            .split_ascii_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

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
            safe_levels += 1;
        }
    }

    safe_levels
}
