//https://adventofcode.com/2022/day/3

// a much cleaner simple solution:  
// https://topaz.github.io/paste/#XQAAAQAOBAAAAAAAAAA6nMjJFMpQiatRvt7HtRFwhPwv9neWyWm+jI5/3aYo9/FVYjYugxGriHffv/nm1U2PaDdCSkYlHpJ3CSlD43LlE1NV87hGxjBXVbhGQBYXDPbnnF3EBsPlGpkJXcpZVRc95smIOae9tJRiF4e/OmW555bRZfLE1KX4LStDAPHXxMfNxELww9fJhvR/ADFSLEIwxtr0h7EbVEi6d5HVI4AmdJUg5JVklAx6tZVGtGJ6JV6MLqttIEk6KLStSJ53KGAGnyH/jMX1b3yTlRbHgriYzNO3f5o3894ntvIaXzyuS8tKs1pOdohdzkP1VdU3K4mSno89/ZBuzO0nFHPLgTuOYqrkVdB1gyUek9PYDkjK2t2yIDahMem64fw6JBybNC96kafNoRkMP3d7Ked8ZPXTvTULIoCwNZ8WSwP+YPjmfuxziMPBnsZwAy191ia23vBpG80y2tJG3ybbHOfEybdv7B2AKYhnzi2BQR/OqviE7iLgLmwaWKOkcsA0ceKY2ij1/+eWoRhFbJQ1EoIJUugWOU6LWZ18SqB3aLw4sXd/Tj/FMcpUrvTJYz7ldU08rZWdFkailI6aUoYnnnRvzP4PSTU=

use std::{collections::HashMap, fs};

fn main() {
    println!("The first star result is {:?}", first_star());
    println!("The second star result is {:?}", second_star());
}

fn first_star() -> i32 {
    let ruck_string = input_to_string();
    let parsed_input: Vec<&str> = ruck_string.split("\r\n").collect();

    let mut result: i32 = 0;
    for sack in parsed_input {
        // Converting string to chars yields ascii codes.
        let sack_chars: Vec<u8> = Vec::from(sack);
        for item_char in &sack_chars[sack_chars.len() / 2..sack_chars.len()] {
            if sack_chars[..sack_chars.len() / 2].contains(item_char) {
                let stringified_char_code = item_char.to_string();
                // Converting these codes to string so they can be parsed as i32s
                let mut item_value = stringified_char_code.parse::<i32>().unwrap();

                // Subtracting enough from ascii codes to get base "priority"
                if item_value > 90 {
                    item_value -= 96;
                } else {
                    item_value -= 38;
                }
                result += item_value;
                break;
            }
        }
    }
    result
}

fn second_star() -> i32 {
    let ruck_string = input_to_string();
    let parsed_input: Vec<&str> = ruck_string.split("\r\n").collect();

    let mut group_counter = 0;
    let mut all_items: Vec<Vec<u8>> = Vec::new();

    let mut result: i32 = 0;
    for sack in parsed_input {
        group_counter += 1;
        let mut sack_chars: Vec<u8> = Vec::from(sack);
        sack_chars.sort_unstable();
        sack_chars.dedup();

        all_items.push(sack_chars);

        if group_counter == 3 {
            let all_items_flattened: Vec<u8> = all_items.into_iter().flatten().collect();
            let mut count_map: HashMap<u8, i32> = HashMap::new();

            for sack_char in all_items_flattened {
                *count_map.entry(sack_char).or_insert(0) += 1;

                if count_map.get(&sack_char).unwrap() == &3 {
                    let stringified_char_code = sack_char.to_string();
                    let mut item_value = stringified_char_code.parse::<i32>().unwrap();

                    if item_value > 90 {
                        item_value -= 96;
                    } else {
                        item_value -= 38;
                    }
                    result += item_value;
                    break;
                }
            }
            all_items = Vec::new();
            group_counter = 0
        }
    }
    result
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\rucksack_contents.txt").unwrap()
}
