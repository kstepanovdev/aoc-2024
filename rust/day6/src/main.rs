use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

#[derive(Debug, PartialEq, Clone)]
pub enum MapElements {
    Obstacle,
    Visited(Direction),
    Empty,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Direction {
    #[default]
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

#[derive(Debug, Default, Clone)]
pub struct GameState {
    map: Vec<Vec<MapElements>>,
    curr_direction: Direction,
    guard_coords: (usize, usize),
}

impl GameState {
    fn step(&self) -> Option<(usize, usize)> {
        let (row, col) = self.guard_coords;
        if row == 0 && self.curr_direction == Direction::Up {
            return None;
        }
        if row == self.map.len() - 1 && self.curr_direction == Direction::Down {
            return None;
        }
        if col == 0 && self.curr_direction == Direction::Left {
            return None;
        }
        if col == self.map[0].len() - 1 && self.curr_direction == Direction::Right {
            return None;
        }

        let new_coords = match self.curr_direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };
        Some(new_coords)
    }

    pub fn is_looped(mut self) -> bool {
        loop {
            let Some((new_row, new_col)) = self.step() else {
                break;
            };

            match self.map[new_row][new_col] {
                MapElements::Obstacle => {
                    self.curr_direction = self.curr_direction.turn_right();
                }
                MapElements::Visited(direction) => {
                    if direction == self.curr_direction {
                        return true;
                    }

                    self.guard_coords = (new_row, new_col);
                }
                MapElements::Empty => {
                    self.map[new_row][new_col] = MapElements::Visited(self.curr_direction);
                    self.guard_coords = (new_row, new_col);
                }
            }
        }

        false
    }
}

fn parse_input(input_file: &str) -> GameState {
    let mut game_state = GameState::default();

    for (row, line) in read_to_string(input_file).unwrap().lines().enumerate() {
        game_state.map.push(vec![]);
        for (col, spot) in line.chars().enumerate() {
            match spot {
                '#' => game_state.map[row].push(MapElements::Obstacle),
                '.' => game_state.map[row].push(MapElements::Empty),
                '^' => {
                    game_state.map[row].push(MapElements::Visited(Direction::Up));
                    game_state.guard_coords = (row, col);
                }
                _ => (),
            }
        }
    }
    game_state
}

fn part_1() -> i32 {
    let mut visited_spots = 0;
    let mut game_state = parse_input("input");

    loop {
        let Some((new_row, new_col)) = game_state.step() else {
            break;
        };

        match game_state.map[new_row][new_col] {
            MapElements::Obstacle => {
                game_state.curr_direction = game_state.curr_direction.turn_right();
            }
            MapElements::Visited(_) => {
                game_state.map[new_row][new_col] = MapElements::Visited(game_state.curr_direction);
                game_state.guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                visited_spots += 1;
                game_state.map[new_row][new_col] = MapElements::Visited(game_state.curr_direction);
                game_state.guard_coords = (new_row, new_col);
            }
        }
    }

    visited_spots + 1
}

fn part_2() -> i32 {
    let mut game_state = parse_input("input");
    let mut looped_count = 0;

    loop {
        let Some((new_row, new_col)) = game_state.step() else {
            break;
        };

        match game_state.map[new_row][new_col].clone() {
            MapElements::Obstacle => {
                game_state.curr_direction = game_state.curr_direction.turn_right();
            }
            MapElements::Visited(direction) => {
                if direction == game_state.curr_direction {
                    looped_count += 1;
                    break;
                }

                game_state.guard_coords = (new_row, new_col);
            }
            MapElements::Empty => {
                {
                    let mut game_state = game_state.clone();
                    game_state.map[new_row][new_col] = MapElements::Obstacle;

                    if game_state.is_looped() {
                        looped_count += 1;
                    }
                }

                game_state.map[new_row][new_col] = MapElements::Visited(game_state.curr_direction);
                game_state.guard_coords = (new_row, new_col);
            }
        }
    }

    looped_count
}
