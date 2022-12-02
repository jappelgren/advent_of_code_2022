use std::fs;

fn main() {
    println!("The first star results are: {:#?}", first_star());
    println!("The second star results are: {:#?}", second_star());
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
    let games = fs::read_to_string(".\\src\\strategy_guide.txt").unwrap();
    let games_collected: Vec<&str> = games.split("\r\n").collect();
    let mut result = 0;

    for game in games_collected {
        let op_shape: char = game.chars().next().unwrap();
        let my_shape: char = game.chars().nth(2).unwrap();
        result += game_result(op_shape, my_shape)
    }

    result
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
        _ => panic!("Invalid argument passed {:#?}", me),
    }
}

fn second_star() -> i32 {
    let games = fs::read_to_string(".\\src\\strategy_guide.txt").unwrap();
    let games_collected: Vec<&str> = games.split("\r\n").collect();
    let mut result = 0;

    for game in games_collected {
        let op_shape: char = game.chars().next().unwrap();
        let my_shape: char = game.chars().nth(2).unwrap();
        result += real_game_result(op_shape, my_shape)
    }

    result
}

fn real_game_result(op: char, me: char) -> i32 {
    let win = ScoreResult { a: 8, b: 9, c: 7 };
    let draw = ScoreResult { a: 4, b: 5, c: 6 };
    let lose = ScoreResult { a: 3, b: 1, c: 2 };

    match me {
        'X' => lose.get(op),
        'Y' => draw.get(op),
        'Z' => win.get(op),
        _ => panic!("Invalid argument passed {:#?}", me),
    }
}
