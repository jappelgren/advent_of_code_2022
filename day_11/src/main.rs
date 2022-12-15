use regex::Regex;
use std::fs;

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    new_operation: String,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: u64,
}

impl Monkey {
    fn new(instructions: String) -> Monkey {
        let re_all_non_digits = Regex::new(r"[^\d\n,.+*]").unwrap();
        let only_digits_commas = re_all_non_digits
            .replace_all(&instructions, "")
            .into_owned();
        let parsed_input: Vec<String> = only_digits_commas
            .split('\n')
            .into_iter()
            .map(|val| val.to_string())
            .collect();

        let items: Vec<u64> = parsed_input[1]
            .split(',')
            .into_iter()
            .map(|item| item.parse().unwrap())
            .collect();
        let new_operation: String = parsed_input[2].to_string();
        let divisor = parsed_input[3].parse().unwrap();
        let true_monkey = parsed_input[4].parse().unwrap();
        let false_monkey = parsed_input[5].parse().unwrap();

        Monkey {
            items,
            new_operation,
            divisor,
            true_monkey,
            false_monkey,
            inspection_count: 0,
        }
    }

    fn evaluate_item(&mut self) -> (u64, usize) {
        let item = self.items[0];
        let (operator, num) = self.new_operation.split_at(1);
        let mut worry_level: u64 = item;
        match operator {
            "*" => {
                if num.is_empty() {
                    worry_level = worry_level * worry_level
                } else {
                    worry_level *= num.to_string().parse::<u64>().unwrap()
                }
            }
            "+" => {
                if num.is_empty() {
                    worry_level = worry_level + worry_level
                } else {
                    worry_level += num.to_string().parse::<u64>().unwrap()
                }
            }
            _ => panic!("Unknown mathematical operator provided {}", operator),
        }
        let next_monkey = if (worry_level / 3) % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        };
        self.items.drain(0..1);
        self.inspection_count += 1;
        (worry_level / 3, next_monkey)
    }

    fn evaluate_item_no_devisor(&mut self, common_denom: u64) -> (u64, usize) {
        let item = self.items[0];
        let (operator, num) = self.new_operation.split_at(1);
        let mut worry_level: u64 = item;

        match operator {
            "*" => {
                if num.is_empty() {
                    worry_level = (worry_level * worry_level) % common_denom
                } else {
                    worry_level =
                        (worry_level * num.to_string().parse::<u64>().unwrap()) % common_denom
                }
            }
            "+" => {
                if num.is_empty() {
                    worry_level = (worry_level + worry_level) % common_denom
                } else {
                    worry_level =
                        (worry_level + num.to_string().parse::<u64>().unwrap()) % common_denom
                }
            }
            _ => panic!("Unknown mathematical operator provided {}", operator),
        }

        let next_monkey = if worry_level % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        };
        self.items.drain(0..1);
        self.inspection_count += 1;
        (worry_level, next_monkey)
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn first_star() -> u64 {
    let input = input_to_string();
    let instructions: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for instruction in instructions {
        monkeys.push(Monkey::new(instruction.to_string()))
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let throw_result = monkeys[i].evaluate_item();
                monkeys[throw_result.1].catch_item(throw_result.0);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys.reverse();
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn second_star() -> u64 {
    let input = input_to_string();
    let instructions: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for instruction in instructions {
        monkeys.push(Monkey::new(instruction.to_string()))
    }

    let mut common_denom = 1;

    for monkey in &monkeys {
        common_denom *= monkey.divisor;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let throw_result = monkeys[i].evaluate_item_no_devisor(common_denom);
                monkeys[throw_result.1].catch_item(throw_result.0);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys.reverse();
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\input.txt").unwrap()
}
