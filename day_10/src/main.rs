use std::fs;

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    second_star();
}

fn first_star() -> i32 {
    let input = input_to_string();
    let instructions: Vec<Vec<&str>> = input
        .split("\r\n")
        .into_iter()
        .map(|inst| inst.split(' ').collect())
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
    }
    result
}

fn cycle_finder(instruction: &str) -> i32 {
    match instruction {
        "addx" => 2,
        "noop" => 1,
        _ => panic!("Function received an unknown instructions {}", instruction),
    }
}

fn second_star() {
    let input = input_to_string();
    let instructions: Vec<Vec<&str>> = input
        .split("\r\n")
        .into_iter()
        .map(|inst| inst.split(' ').collect())
        .collect();
    whats_on_the_screen(instructions);
}

fn whats_on_the_screen(instructions: Vec<Vec<&str>>) {
    let mut screen = format!("");
    let screen_length: i32 = screen.clone().len().to_string().parse().unwrap();
    let mut cycle_count = 0;
    let mut cycle_left = 0;
    let mut register: i32 = 1;

    for instruction in instructions {
        cycle_left = cycle_finder(instruction.first().unwrap());

        while cycle_left > 0 {
            let sub = sub(&cycle_count);
            if screen.clone().len().to_string().parse::<i32>().unwrap() - sub == register - 1
                || screen.clone().len().to_string().parse::<i32>().unwrap() - sub == register
                || screen.clone().len().to_string().parse::<i32>().unwrap() - sub == register + 1
            {
                screen.push_str("#");
            } else {
                screen.push_str(".");
            }
            cycle_left -= 1;
            cycle_count += 1;
        }

        if instruction.len() == 2 {
            register += instruction.last().unwrap().parse::<i32>().unwrap()
        }
    }
    parse_result(screen);
}

fn sub(cycle: &i32) -> i32 {
    match cycle {
        0..=39 => 0,
        40..=79 => 40,
        80..=119 => 80,
        120..=159 => 120,
        160..=199 => 160,
        200..=240 => 200,
        _ => panic!("Value out of range.  Must be lower than 240, got {}", cycle),
    }
}

fn parse_result(result: String) {
    let (one, two_three_four_five_six) = result.split_at(40);
    let (two, three_four_five_six) = two_three_four_five_six.split_at(40);
    let (three, four_five_six) = three_four_five_six.split_at(40);
    let (four, five_six) = four_five_six.split_at(40);
    let (five, six) = five_six.split_at(40);


    let result = format!("{}\n{}\n{}\n{}\n{}\n{}", one, two, three, four, five, six);
    println!("The result for the second star is:\n {}", result);
    
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\input.txt").unwrap()
}
