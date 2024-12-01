use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let mut lefts = vec![];
    let mut rights = vec![];
    let mut distance = 0;

    for file in read_to_string("input").unwrap().lines() {
        let mut line = file.split_ascii_whitespace();

        lefts.push(line.next().unwrap().parse::<i32>().unwrap());
        rights.push(line.next().unwrap().parse::<i32>().unwrap());
    }
    lefts.sort();
    rights.sort();

    for i in 0..lefts.len() {
        distance += (lefts[i] - rights[i]).abs();
    }

    distance
}

fn part_2() -> i32 {
    let mut lefts = Vec::new();
    let mut rights = HashMap::new();

    for file in read_to_string("input").unwrap().lines() {
        let mut line = file.split_ascii_whitespace();

        lefts.push(line.next().unwrap().parse::<i32>().unwrap());
        let right_value = line.next().unwrap().parse::<i32>().unwrap();

        let value = rights.entry(right_value).or_insert(0);
        *value += 1;
    }

    let mut result = 0;
    for key in lefts.iter() {
        if let Some(value) = rights.get(key) {
            result += key * value;
        }
    }

    result
}
