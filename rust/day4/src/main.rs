use std::fs::read_to_string;
pub const KEY: &str = "XMAS";
pub const CROSS_MAS: &str = "MAS";

pub enum Direction {
    Horizontal,
    Vertical,
    DiagonalRightUp,
    DiagonalRightDown,
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
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
                            key = puzzle[row][col..col + KEY.len()].iter().collect::<String>();
                        }
                    }
                    Direction::Vertical => {
                        if row + KEY.len() <= puzzle.len() {
                            key = (row..row + KEY.len())
                                .map(|r| puzzle[r][col])
                                .collect::<String>();
                        }
                    }
                    Direction::DiagonalRightUp => {
                        if row + 1 >= KEY.len() && col + KEY.len() <= puzzle.len() {
                            key = (0..KEY.len())
                                .map(|k| puzzle[row - k][col + k])
                                .collect::<String>();
                        }
                    }
                    Direction::DiagonalRightDown => {
                        if col + KEY.len() <= puzzle[0].len() && row + KEY.len() <= puzzle.len() {
                            key = (0..KEY.len())
                                .map(|k| puzzle[row + k][col + k])
                                .collect::<String>();
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

fn part_2() -> i32 {
    let mut encounters = 0;
    let puzzle = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            let mut cross = [false, false];
            for (idx, direction) in [Direction::DiagonalRightUp, Direction::DiagonalRightDown]
                .iter()
                .enumerate()
            {
                let mut key = vec![];
                if puzzle[row][col] != 'A' {
                    continue;
                }
                if !(row > 0 && col > 0 && row + 1 < puzzle.len() && col + 1 < puzzle.len()) {
                    continue;
                }

                match direction {
                    Direction::Horizontal | Direction::Vertical => {
                        unreachable!()
                    }
                    Direction::DiagonalRightUp => {
                        if row > 0 && col > 0 && row + 1 < puzzle.len() && col + 1 < puzzle.len() {
                            key.push(puzzle[row - 1][col - 1]);
                            key.push(puzzle[row + 1][col + 1]);
                        }
                    }
                    Direction::DiagonalRightDown => {
                        if row > 0 && col > 0 && row + 1 < puzzle.len() && col + 1 < puzzle.len() {
                            key.push(puzzle[row - 1][col + 1]);
                            key.push(puzzle[row + 1][col - 1]);
                        }
                    }
                };

                if key.contains(&'M') && key.contains(&'S') {
                    cross[idx] = true;
                }
                if cross.iter().all(|&x| x) {
                    encounters += 1;
                }
            }
        }
    }

    encounters
}
