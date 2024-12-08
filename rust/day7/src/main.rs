use std::fs::read_to_string;
use std::vec;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> u64 {
    let mut equations: Vec<(u64, Vec<u64>)> = vec![];

    for file in read_to_string("input").unwrap().lines() {
        let (production, options) = file.split_once(":").unwrap();

        let production = production.parse::<u64>().unwrap();
        let options = options
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        equations.push((production, options));
    }

    equations.iter().fold(0, |acc, (goal, equation)| {
        acc + if try_to_solve(goal, equation) {
            *goal
        } else {
            0
        }
    })
}

pub fn try_to_solve(goal: &u64, nums: &[u64]) -> bool {
    if nums.len() == 1 {
        return *goal == nums[0];
    }

    let (&last, rest) = nums.split_last().unwrap();

    goal % last == 0 && try_to_solve(&(goal / last), rest)
        || goal > &last && try_to_solve(&(goal - last), rest)
}

pub fn try_to_solve_part_2(subgoal: &u64, goal: &u64, nums: &[u64]) -> bool {
    if subgoal > goal {
        return false;
    }

    if nums.is_empty() {
        return subgoal == goal;
    }

    let (&first, rest) = nums.split_first().unwrap();
    let concat = format!("{}{}", subgoal, first).parse::<u64>().unwrap();

    try_to_solve_part_2(&(subgoal + first), goal, rest)
        || try_to_solve_part_2(&(subgoal * first), goal, rest)
        || try_to_solve_part_2(&concat, goal, rest)
}

fn part_2() -> u64 {
    let mut equations: Vec<(u64, Vec<u64>)> = vec![];

    for file in read_to_string("input").unwrap().lines() {
        let (production, options) = file.split_once(":").unwrap();

        let production = production.parse::<u64>().unwrap();
        let options = options
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        equations.push((production, options));
    }

    equations.iter().fold(0, |acc, (goal, equation)| {
        let (first, rest) = equation.split_first().unwrap();
        acc + if try_to_solve_part_2(first, goal, rest) {
            *goal
        } else {
            0
        }
    })
}
