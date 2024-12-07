use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn read_file_and_split_to_groups() -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
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

    let all_pages = file[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, all_pages)
}

fn part_1() -> i32 {
    let (rules, all_pages) = read_file_and_split_to_groups();

    let mut result = 0;

    for pages in all_pages {
        let mut valid = true;
        let mut passed_values = Vec::new();
        for page in &pages {
            if let Some(forbidden_pages) = rules.get(&page) {
                if let Some(_) = passed_values.iter().find(|x| forbidden_pages.contains(x)) {
                    valid = false;
                    break;
                }
            }
            passed_values.push(*page);
        }
        if valid {
            result += pages[pages.len() / 2];
        }
    }

    result
}

fn part_2() -> i32 {
    let (rules, all_pages) = read_file_and_split_to_groups();
    let mut result = 0;

    for mut pages in all_pages {
        let mut valid = true;
        let mut passed_values = Vec::new();
        let mut page_idx = 0;

        while page_idx < pages.len() {
            let page = pages[page_idx];
            if let Some(forbidden_pages) = rules.get(&page) {
                if let Some(intersected) =
                    passed_values.iter().find(|x| forbidden_pages.contains(x))
                {
                    valid = false;
                    let intersected_idx = pages.iter().position(|x| x == intersected).unwrap();
                    pages.swap(page_idx, intersected_idx);
                    passed_values.clear();
                    page_idx = 0;
                    continue;
                }
            }
            passed_values.push(page);
            page_idx += 1;
        }

        if !valid {
            result += pages[pages.len() / 2];
        }
    }

    result
}
