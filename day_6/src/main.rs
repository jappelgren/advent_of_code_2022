// https://adventofcode.com/2022/day/6
use std::fs;

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}

fn first_star() -> usize {
    let input: String = input_to_string();
    let input_vec: Vec<char> = input.chars().collect();

    let mut i: usize = 3;

    while i < input_vec.len() {
        let mut cur_slice = vec![
            input_vec[i - 3],
            input_vec[i - 2],
            input_vec[i - 1],
            input_vec[i],
        ];
        cur_slice.sort_unstable();
        cur_slice.dedup();

        if cur_slice.len() == 4 {
            return i + 1;
        }

        i += 1;
    }

    0
}

fn second_star() -> usize {
    let input: String = input_to_string();
    let input_vec: Vec<char> = input.chars().collect();

    let mut i: usize = 13;

    while i < input_vec.len() {
        let mut cur_slice: Vec<char> = Vec::new();

        for item in input_vec.iter().take(i + 1).skip(i - 13) {
            cur_slice.push(*item);
        }
        cur_slice.reverse();
        cur_slice.sort_unstable();
        cur_slice.dedup();

        if cur_slice.len() == 14 {
            return i + 1;
        }

        i += 1;
    }

    0
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\signal.txt").unwrap()
}
