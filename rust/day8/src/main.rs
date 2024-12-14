use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::{Add, Sub};

use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part_2());
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

    fn check_bounds(&self, map: &Vec<Vec<char>>) -> bool {
        self.x >= 0 && self.x < map.len() as i32 && self.y >= 0 && self.y < map[0].len() as i32
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

fn part_1() -> i32 {
    let mut map = vec![];
    let mut locations: HashMap<char, HashSet<Location>> = HashMap::new();
    let mut resonanses = HashSet::new();

    for (x, line) in read_to_string("input").unwrap().lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        map.push(vec![]);
        for (y, c) in line.trim().chars().enumerate() {
            map[x].push(c);

            if c != '.' {
                locations
                    .entry(c)
                    .or_default()
                    .insert(Location::new(x as i32, y as i32));
            }
        }
    }

    for (_, positions) in locations {
        for pairs in positions.iter().combinations(2) {
            let location = *pairs[0];
            let neighbour = *pairs[1];

            let resonance_1 = location + (location - neighbour);
            let resonance_2 = neighbour + (neighbour - location);

            if resonance_1.check_bounds(&map) {
                resonanses.insert(resonance_1);
            };
            if resonance_2.check_bounds(&map) {
                resonanses.insert(resonance_2);
            };
        }
    }

    resonanses.len() as i32
}
