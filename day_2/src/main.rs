use std::fs;

fn main() {
    println!("The first star results are: {:#?}", first_star());
}

// Scoring
// Selected Shape + Outcome

// Shapes:
// A <- Rock: 1 -> X
// B <- Paper: 2 -> Y
// C <- Scissors: 3 -> Z

// Outcome
// Win: 6
// Draw: 3
// Lose: 0

fn first_star() -> i32 {
    let games_collected = parse_input();
    let mut result = 0;

    for game in games_collected {
        let op_shape: char = game.chars().next().unwrap();
        let my_shape: char = game.chars().nth(2).unwrap();
        result += game_result(op_shape, my_shape)
    }

    result
}

fn parse_input() -> Vec<&str> {
    let games = fs::read_to_string(".\\src\\strategy_guide.txt").unwrap();
    let games_collected = games.split("\r\n").collect();
    games_collected
}

struct ScoreResult {
    a: i32,
    b: i32,
    c: i32,
}

impl ScoreResult {
    fn get(&self, op_shape: char) -> i32 {
        match op_shape {
            'A' => self.a,
            'B' => self.b,
            'C' => self.c,
            _ => panic!("Invalid argument passed {:#?}", op_shape),
        }
    }
}

fn game_result(op: char, me: char) -> i32 {
    let rock = ScoreResult { a: 4, b: 1, c: 7 };
    let paper = ScoreResult { a: 8, b: 5, c: 2 };
    let scissor = ScoreResult { a: 3, b: 9, c: 6 };

    match me {
        'X' => rock.get(op),
        'Y' => paper.get(op),
        'Z' => scissor.get(op),
        _ => panic!("Invalid argument passed {:#?}", op),
    }
}

fn second_star() {

}

fn real_game_results() {

}