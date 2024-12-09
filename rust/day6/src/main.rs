use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

#[derive(Debug, PartialEq, Clone)]
pub enum MapElements {
    Guard,
    Obstacle,
    Visited(Direction),
    Empty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn step(
    direction: Direction,
    curr_guard_coords: (usize, usize),
    map: &[Vec<MapElements>],
) -> Option<(usize, usize)> {
    let (row, col) = curr_guard_coords;
    if row == 0 && direction == Direction::Up {
        return None;
    }
    if row == map.len() - 1 && direction == Direction::Down {
        return None;
    }
    if col == 0 && direction == Direction::Left {
        return None;
    }
    if col == map[0].len() - 1 && direction == Direction::Right {
        return None;
    }

    let new_coords = match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    };
    Some(new_coords)
}

fn part_1() -> i32 {
    let mut map = vec![];
    let mut curr_direction = Direction::Up;
    let mut guard_coords = (0, 0);
    let mut visited_spots = 0;

    for (row, line) in read_to_string("input").unwrap().lines().enumerate() {
        map.push(vec![]);
        for (col, spot) in line.chars().enumerate() {
            match spot {
                '#' => map[row].push(MapElements::Obstacle),
                '.' => map[row].push(MapElements::Empty),
                '^' => {
                    map[row].push(MapElements::Guard);
                    guard_coords = (row, col);
                }
                _ => (),
            }
        }
    }

    loop {
        let (prev_row, prev_col) = guard_coords;
        let Some((new_row, new_col)) = step(curr_direction, guard_coords, &map) else {
            break;
        };

        match map[new_row][new_col] {
            MapElements::Guard => {
                unreachable!("Guard found at: ({}, {})", new_row, new_col);
            }
            MapElements::Obstacle => {
                curr_direction = curr_direction.turn_right();
            }
            MapElements::Visited(_) => {
                map[new_row][new_col] = MapElements::Guard;
                map[prev_row][prev_col] = MapElements::Visited(curr_direction);
                guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                map[new_row][new_col] = MapElements::Guard;
                map[prev_row][prev_col] = MapElements::Visited(curr_direction);
                visited_spots += 1;
                guard_coords = (new_row, new_col);
            }
        }
    }

    visited_spots + 1
}

fn part_2() -> i32 {
    let mut map = vec![];
    let mut curr_direction = Direction::Up;
    let mut guard_coords = (0, 0);
    let mut looped_count = 0;

    for (row, line) in read_to_string("input").unwrap().lines().enumerate() {
        map.push(vec![]);
        for (col, spot) in line.chars().enumerate() {
            match spot {
                '#' => map[row].push(MapElements::Obstacle),
                '.' => map[row].push(MapElements::Empty),
                '^' => {
                    map[row].push(MapElements::Visited(Direction::Up));
                    guard_coords = (row, col);
                }
                _ => (),
            }
        }
    }

    loop {
        let Some((new_row, new_col)) = step(curr_direction, guard_coords, &map) else {
            break;
        };

        match map[new_row][new_col].clone() {
            MapElements::Guard => {
                unreachable!("Guard found at: ({}, {})", new_row, new_col);
            }
            MapElements::Obstacle => {
                curr_direction = curr_direction.turn_right();
            }
            MapElements::Visited(direction) => {
                if direction == curr_direction {
                    looped_count += 1;
                    break;
                }

                guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                {
                    let mut cloned_map = map.clone();
                    cloned_map[new_row][new_col] = MapElements::Obstacle;
                    if is_looped(cloned_map, guard_coords, curr_direction) {
                        looped_count += 1;
                    }
                }

                map[new_row][new_col] = MapElements::Visited(curr_direction);
                guard_coords = (new_row, new_col);
            }
        }
    }

    looped_count
}

pub fn is_looped(
    mut map: Vec<Vec<MapElements>>,
    mut guard_coords: (usize, usize),
    mut curr_direction: Direction,
) -> bool {
    loop {
        let Some((new_row, new_col)) = step(curr_direction, guard_coords, &map) else {
            break;
        };

        match map[new_row][new_col].clone() {
            MapElements::Guard => {
                unreachable!("Guard found at: ({}, {})", new_row, new_col);
            }
            MapElements::Obstacle => {
                curr_direction = curr_direction.turn_right();
            }
            MapElements::Visited(direction) => {
                if direction == curr_direction {
                    return true;
                }

                guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                map[new_row][new_col] = MapElements::Visited(curr_direction);
                guard_coords = (new_row, new_col);
            }
        }
    }

    false
}
