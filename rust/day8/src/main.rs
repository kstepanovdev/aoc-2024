use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::{Add, AddAssign, Sub};

use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    fn check_bounds(&self, state: &PuzzleState) -> bool {
        self.x >= 0 && self.x < state.width && self.y >= 0 && self.y < state.height
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, other: Location) -> Location {
        Location::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Location::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Location {
    fn add_assign(&mut self, other: Location) {
        *self = Location::new(self.x + other.x, self.y + other.y);
    }
}

fn read_input() -> PuzzleState {
    let mut state = PuzzleState::default();

    for (x, line) in read_to_string("input").unwrap().lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        state.width = line.len() as i32;
        for (y, c) in line.trim().chars().enumerate() {
            if c != '.' {
                state
                    .locations
                    .entry(c)
                    .or_default()
                    .insert(Location::new(x as i32, y as i32));
            }
        }
        state.height = (x + 1) as i32;
    }
    state
}

#[derive(Debug, Default, Clone)]
pub struct PuzzleState {
    height: i32,
    width: i32,
    locations: HashMap<char, HashSet<Location>>,
    resonanses: HashSet<Location>,
}

fn part_1() -> i32 {
    let mut state = read_input();

    for (_, positions) in state.locations.iter() {
        for pairs in positions.iter().combinations(2) {
            let location = *pairs[0];
            let neighbour = *pairs[1];

            let resonance_1 = location + (location - neighbour);
            let resonance_2 = neighbour + (neighbour - location);

            if resonance_1.check_bounds(&state) {
                state.resonanses.insert(resonance_1);
            };
            if resonance_2.check_bounds(&state) {
                state.resonanses.insert(resonance_2);
            };
        }
    }

    state.resonanses.len() as i32
}

fn part_2() -> i32 {
    let mut state = read_input();

    for (_, positions) in state.locations.iter() {
        for pairs in positions.iter().combinations(2) {
            let location = *pairs[0];
            let neighbour = *pairs[1];

            let resonanse_1_diff = location - neighbour;
            let resonanse_2_diff = neighbour - location;

            let mut resonance_1 = location;
            while resonance_1.check_bounds(&state) {
                state.resonanses.insert(resonance_1);
                resonance_1 += resonanse_1_diff;
            }
            let mut resonance_1 = location;
            while resonance_1.check_bounds(&state) {
                state.resonanses.insert(resonance_1);
                resonance_1 += resonanse_2_diff;
            }
        }
    }

    state.resonanses.len() as i32
}
