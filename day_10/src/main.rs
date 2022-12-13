use std::fs;

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}

fn first_star() -> i32 {
    let input = input_to_string();
    let instructions: Vec<Vec<&str>> = input
        .split("\r\n")
        .into_iter()
        .map(|inst| {
            inst.split(' ').collect()
        })
        .collect();
        process_instructions(instructions)
    
}

fn process_instructions(instructions: Vec<Vec<&str>>) -> i32 {
    let mut result = 0;
    let mut until_signal_calculation = 20;
    let mut next_signal_calculation = 20;
    let mut cycle_counter = 0;
    let mut register = 1;

    for instruction in instructions {
        cycle_counter = cycle_finder(instruction.first().unwrap());
        while cycle_counter > 0 {
            cycle_counter -= 1;
            until_signal_calculation -= 1;
            if until_signal_calculation == 0 {
                result += register * next_signal_calculation;
                next_signal_calculation += 40;
                until_signal_calculation = 40
            }
        }
        if instruction.len() == 2 {
            register += instruction.last().unwrap().parse::<i32>().unwrap()
        }
        println!("{}", next_signal_calculation);
    }
    result
}

fn cycle_finder(instruction: &str) -> i32 {
    match instruction {
        "addx" => 2,
        "noop" => 1,
        _ => panic!("Function received an unknown instructions {}", instruction)
    }
}

fn second_star() {}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\input.txt").unwrap()
}
