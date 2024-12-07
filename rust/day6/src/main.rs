use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part_2());
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MapElements {
    Guard,
    Obstacle,
    Visited,
    Empty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    pub fn turn_right(&self) -> Directions {
        match self {
            Directions::Up => Directions::Right,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
            Directions::Right => Directions::Down,
        }
    }
}

fn step(
    direction: Directions,
    curr_guard_coords: (usize, usize),
    map: &[Vec<MapElements>],
) -> Option<(usize, usize)> {
    let (row, col) = curr_guard_coords;
    if row == 0 && direction == Directions::Up {
        return None;
    }
    if row == map.len() - 1 && direction == Directions::Down {
        return None;
    }
    if col == 0 && direction == Directions::Left {
        return None;
    }
    if col == map[0].len() - 1 && direction == Directions::Right {
        return None;
    }

    let new_coords = match direction {
        Directions::Up => (row - 1, col),
        Directions::Down => (row + 1, col),
        Directions::Left => (row, col - 1),
        Directions::Right => (row, col + 1),
    };
    Some(new_coords)
}

fn part_1() -> i32 {
    let mut map = vec![];
    let mut cur_direction = Directions::Up;
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
        let Some((new_row, new_col)) = step(cur_direction, guard_coords, &map) else {
            break;
        };

        match map[new_row][new_col] {
            MapElements::Guard => {
                unreachable!("Guard found at: ({}, {})", new_row, new_col);
            }
            MapElements::Obstacle => {
                cur_direction = cur_direction.turn_right();
            }
            MapElements::Visited => {
                map[new_row][new_col] = MapElements::Guard;
                map[prev_row][prev_col] = MapElements::Visited;
                guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                map[new_row][new_col] = MapElements::Guard;
                map[prev_row][prev_col] = MapElements::Visited;
                visited_spots += 1;
                guard_coords = (new_row, new_col);
            }
        }
    }

    visited_spots + 1
}

fn part_2() -> i32 {
    todo!()
}
