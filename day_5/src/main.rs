use regex::Regex;
use std::fs;

fn main() {
    println!(
        "The result for the first star is: {:?}",
        move_them(String::from("one"))
    );
    println!(
        "The result for the second star is: {:?}",
        move_them(String::from("two"))
    );
}

fn move_them(star: String) -> Vec<char> {
    let instructions: String = input_to_string();
    let re = Regex::new(r"[ \D ]{4,6}").unwrap();
    let instructions_parsed: Vec<String> = instructions
        .split("\r\nmove ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|instructions| re.replace_all(instructions, ",").into_owned())
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![
        vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'],
        vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'],
        vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'],
        vec!['V', 'S', 'T', 'D', 'F'],
        vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'],
        vec!['Z', 'C', 'L', 'S'],
        vec!['Z', 'B', 'M', 'V', 'D', 'F'],
        vec!['T', 'J', 'B'],
        vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'],
    ];

    for instruction in instructions_parsed {
        if star == "one" {
            move_one(&mut stacks, instruction);
        } else {
            move_stack(&mut stacks, instruction);
        }
    }

    let mut result: Vec<char> = Vec::new();

    for stack in stacks {
        result.push(stack[stack.len() - 1])
    }

    result
}

fn move_stack(stacks: &mut Vec<Vec<char>>, instructions: String) {
    let parsed_instructions: Vec<usize> = instructions
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let pick_up_point: usize =
        stacks[parsed_instructions[1] - 1].clone().len() - parsed_instructions[0];
    let stack_to_move = stacks[parsed_instructions[1] - 1].split_off(pick_up_point);
    stacks[parsed_instructions[2] - 1].extend_from_slice(&stack_to_move);
}

fn move_one(stacks: &mut Vec<Vec<char>>, instructions: String) {
    let parsed_instructions: Vec<usize> = instructions
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    for _ in 0..parsed_instructions[0] {
        let crate_to_move = stacks[parsed_instructions[1] - 1].pop().unwrap();
        stacks[parsed_instructions[2] - 1].push(crate_to_move);
    }
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\moves.txt").unwrap()
}
