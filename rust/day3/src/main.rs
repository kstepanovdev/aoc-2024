use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;

    let file = read_to_string("input").unwrap();
    for (_, [left, right]) in re.captures_iter(&file).map(|c| c.extract()) {
        result += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }

    result
}

fn part_2() -> i32 {
    let re = regex::Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;

    let file = read_to_string("input").unwrap();

    let mut calc_allowed = true;
    for re_match in re.find_iter(&file) {
        let re_match = re_match.as_str();
        if re_match == "don't()" {
            calc_allowed = false;
            continue;
        } else if re_match == "do()" {
            calc_allowed = true;
            continue;
        }

        if !calc_allowed {
            continue;
        }

        let re = regex::Regex::new(r"\((\d{1,3}),(\d{1,3})\)").unwrap();
        for (_, [left, right]) in re.captures_iter(re_match).map(|c| c.extract()) {
            result += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
    }

    result
}
