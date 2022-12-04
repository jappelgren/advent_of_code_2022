// https://adventofcode.com/2022/day/4

use std::fs;

fn main() {
    println!("The result for star one is: {:?}", first_star())
}

fn first_star() -> i32 {
    let stringified_input: String = input_to_string();
    let split_input: Vec<Vec<&str>> = stringified_input
        .split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|areas| areas.split(',').collect())
        .collect();
    let mut result = 0;
    for range in split_input {
        let mut parsed_range_set: Vec<Vec<i32>> = Vec::new();
        for single_range in range {
            parsed_range_set.push(create_range(single_range));
        }

        let larger_len = larger_len(&parsed_range_set[0], &parsed_range_set[1]);
        let mut flattened_sets: Vec<i32> = parsed_range_set.into_iter().flatten().collect();
        flattened_sets.sort_unstable();
        flattened_sets.dedup();

        if flattened_sets.len() == larger_len {
            result += 1;
        }
    }
    result
}

fn create_range(range_str: &str) -> Vec<i32> {
    let split_range: Vec<i32> = range_str
        .split('-')
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.parse().unwrap())
        .collect();
    (split_range[0]..=split_range[1]).collect()
}

fn larger_len(first_vec: &Vec<i32>, second_vec: &Vec<i32>) -> usize {
    if first_vec.len() > second_vec.len() {
        first_vec.len()
    } else {
        second_vec.len()
    }
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\areas_to_clean.txt").unwrap()
}
