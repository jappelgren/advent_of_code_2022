use std::fs;

fn main() {
    println!("The first star result is {:?}", first_star());
}

fn first_star() -> i32 {
    let ruck_string = input_to_string();
    let parsed_input: Vec<&str> = ruck_string.split("\r\n").collect();

    let mut result: i32 = 0;
    for sack in parsed_input {
        let sack_chars:Vec<u8> = Vec::from(sack);
        for item_char in &sack_chars[sack_chars.len() / 2..sack_chars.len()] {
            if sack_chars[..sack_chars.len() / 2].contains(item_char) {
                let stringified_char_code = item_char.to_string();
                let mut item_value = stringified_char_code.parse::<i32>().unwrap();

                if item_value > 90 {
                    item_value = item_value - 96;
                } else {
                    item_value = item_value - 38;
                }
                result += item_value;
                break;
            }
        }
    }
    result
}

fn input_to_string() -> String {
    let stringified_input = fs::read_to_string(".\\src\\rucksack_contents.txt").unwrap();
    stringified_input
}