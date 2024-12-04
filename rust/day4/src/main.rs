use std::fs::read_to_string;
pub const KEY: &str = "XMAS";

pub enum Direction {
    Horizontal,
    Vertical,
    DiagonalRightUp,
    DiagonalRightDown,
}

fn main() {
    println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let directions = vec![
        Direction::Horizontal,
        Direction::Vertical,
        Direction::DiagonalRightUp,
        Direction::DiagonalRightDown,
    ];

    let mut encounters = 0;
    let puzzle = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            for direction in directions.iter() {
                let mut key = String::new();
                match direction {
                    Direction::Horizontal => {
                        if col + KEY.len() <= puzzle[0].len() {
                            for k in 0..KEY.len() {
                                key.push(puzzle[row][col + k]);
                            }
                        }
                    }
                    Direction::Vertical => {
                        if row + KEY.len() <= puzzle.len() {
                            for k in 0..KEY.len() {
                                key.push(puzzle[row + k][col]);
                            }
                        }
                    }
                    Direction::DiagonalRightUp => {
                        if row + 1 >= KEY.len() && col + KEY.len() <= puzzle.len() {
                            for k in 0..KEY.len() {
                                key.push(puzzle[row - k][col + k]);
                            }
                        }
                    }
                    Direction::DiagonalRightDown => {
                        if col + KEY.len() <= puzzle[0].len() && row + KEY.len() <= puzzle.len() {
                            for k in 0..KEY.len() {
                                key.push(puzzle[row + k][col + k]);
                            }
                        }
                    }
                };
                if key == KEY || key.chars().rev().collect::<String>() == KEY {
                    encounters += 1;
                }
            }
        }
    }

    encounters
}

// fn part_2() -> i32 {}
