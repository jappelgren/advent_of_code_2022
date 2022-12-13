// https://adventofcode.com/2022/day/9
use std::fs;

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}

#[derive(Clone)]

struct PositionHistory {
    position: Vec<Position>,
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn first_star() -> usize {
    let input: String = input_to_string();
    let input_parsed: Vec<(char, i32)> = input
        .split("\r\n")
        .into_iter()
        .map(|instruction| {
            (
                instruction.split_at(2).0.chars().next().unwrap(),
                instruction.split_at(2).1.parse().unwrap(),
            )
        })
        .collect();

    let mut head_position: PositionHistory = PositionHistory {
        position: vec![Position { x: 0, y: 0 }],
    };
    let mut tail_position: PositionHistory = PositionHistory {
        position: vec![Position { x: 0, y: 0 }],
    };

    for instruction in input_parsed {
        move_head(&mut head_position, instruction.0, instruction.1)
    }
    move_tail(&mut tail_position, &head_position);

    tail_position.position.sort_unstable();
    tail_position.position.dedup();

    tail_position.position.len()
}

fn move_head(head_position: &mut PositionHistory, direction: char, steps: i32) {
    for _i in 1..=steps {
        let x = head_position.position.last().unwrap().x;
        let y = head_position.position.last().unwrap().y;

        let movement_result = head_movement(direction, x, y);
        let new_position: Position = Position {
            x: movement_result.0,
            y: movement_result.1,
        };
        head_position.position.push(new_position);
    }
}

fn move_tail(tail_position: &mut PositionHistory, head_position: &PositionHistory) {
    for position in &head_position.position {
        let head_x = position.x;
        let head_y = position.y;

        let tail_x = tail_position.position.last().unwrap().x;
        let tail_y = tail_position.position.last().unwrap().y;

        let movement_result = tail_movement(head_x, head_y, tail_x, tail_y);
        let new_position: Position = Position {
            x: movement_result.0,
            y: movement_result.1,
        };
        if tail_position.position.last().unwrap() != &new_position {
            tail_position.position.push(new_position);
        }
    }
}

fn head_movement(direction: char, x: i32, y: i32) -> (i32, i32) {
    match direction {
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _ => panic!(
            "Direction must be 'U', 'D', 'L', or 'R'. Received {}",
            direction
        ),
    }
}

fn tail_movement(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let x_diff = tail_x - head_x;
    let y_diff = tail_y - head_y;

    match (x_diff, y_diff) {
        (-2,-2) => (head_x - 1, head_y - 1),
        (-2, 2) => (head_x - 1, head_y + 1),
        (2, -2) => (head_x + 1, head_y - 1),
        (2, 2) => (head_x + 1, head_y + 1),
        (-2, _) => (head_x - 1, head_y),
        (2, _) => (head_x + 1, head_y),
        (_, -2) => (head_x, head_y - 1),
        (_, 2) => (head_x, head_y + 1),
        _ => (tail_x, tail_y)
    }
}

fn second_star() -> usize {
    let input: String = input_to_string();
    let input_parsed: Vec<(char, i32)> = input
        .split("\r\n")
        .into_iter()
        .map(|instruction| {
            (
                instruction.split_at(2).0.chars().next().unwrap(),
                instruction.split_at(2).1.parse().unwrap(),
            )
        })
        .collect();
    let mut rope = create_rope(10);
    for instruction in input_parsed {
        move_head(&mut rope[0], instruction.0, instruction.1);
    }
    for i in 1..rope.len() {
        let (head, tail) = rope.split_at_mut(i);
        move_tail(&mut tail[0], head.last().unwrap())
    }

    let mut tail_sorted: PositionHistory = rope.last().unwrap().clone();

    tail_sorted.position.sort_unstable();
    tail_sorted.position.dedup();

    tail_sorted.position.len()
}

fn create_rope(knots: i32) -> Vec<PositionHistory> {
    let mut rope: Vec<PositionHistory> = Vec::new();
    for _ in 0..knots {
        let start_pos: Position = Position { x: 0, y: 0 };
        let knot: PositionHistory = PositionHistory {
            position: vec![start_pos],
        };
        rope.push(knot)
    }
    rope
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\input.txt").unwrap()
}
