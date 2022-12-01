use std::fs;

fn first_star() -> Vec<i32> {
    let calories: String = fs::read_to_string(".\\src\\calories.txt").unwrap().replace("\r\n\r", ",").replace('\n', "");
    let cals_parsed: Vec<&str> = calories.split(',').collect();
    let mut cal_vec: Vec<i32> = Vec::new();
    
    for cals in cals_parsed {
        let further_parsed_cals: String = cals.replace('\r', ",");
        let reduced_cals: i32 = further_parsed_cals.split(',').map(|cal| cal.parse::<i32>().unwrap()).reduce(|a,b| a + b).unwrap();
        cal_vec.push(reduced_cals);
    }

    cal_vec.sort();
    cal_vec
}

fn second_star(sorted_cals: Vec<i32>) -> i32 {
    return sorted_cals.iter().rev().take(3).sum();
}

fn main() {
    let sorted_cals: Vec<i32> = first_star();
    println!("The largest amount of calories carried by one elf is {:#?}.", sorted_cals[sorted_cals.len() - 1]);
    
    let summed_top_three: i32 = second_star(sorted_cals);
    println!("The sum of the top three elves calories is {:#?}.", summed_top_three);
}
