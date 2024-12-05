use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut result = 0;

    let re = regex::Regex::new(r"(\d{2})\|(\d{2})").unwrap();

    let binding = read_to_string("input").unwrap();
    let file: Vec<&str> = binding.split("\n\n").collect();

    for rule in file[0].lines() {
        for (_, [left, right]) in re.captures_iter(&rule).map(|c| c.extract()) {
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();

            let set_by_value = rules.entry(left).or_insert(HashSet::new());
            set_by_value.insert(right);
        }
    }

    'outer: for pages in file[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
    {
        let mut passed_values = HashSet::new();
        for page in &pages {
            let forbidden_pages = rules.get(&page);

            match forbidden_pages {
                Some(forbidden_pages) => {
                    if passed_values.intersection(forbidden_pages).count() == 0 {
                        passed_values.insert(*page);
                    } else {
                        break 'outer;
                    }
                }
                None => {
                    passed_values.insert(*page);
                }
            }
        }

        result += pages[pages.len() / 2];
        println!("{:?}", pages);
    }

    result
}

fn part_2() -> i32 {
    todo!()
}
