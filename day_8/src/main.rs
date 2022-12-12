// https://adventofcode.com/2022/day/8
use std::{collections::HashMap, fs};

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}
#[derive(Debug, Clone)]
struct Tree {
    height: i32,
    visible: bool,
    scenic_score: HashMap<Direction, i32>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Tree {
    fn set_visible(&mut self, visibility: bool) {
        self.visible = visibility;
    }

    fn set_direction_score(&mut self, dir: Direction, score: i32) {
        self.scenic_score.insert(dir, score);
    }

    fn calculate_score(&self) -> i32 {
        let mut score_vec: Vec<i32> = Vec::new();

        if self.scenic_score.contains_key(&Direction::NORTH) {
            score_vec.push(*self.scenic_score.get(&Direction::NORTH).unwrap())
        }
        if self.scenic_score.contains_key(&Direction::SOUTH) {
            score_vec.push(*self.scenic_score.get(&Direction::SOUTH).unwrap())
        }
        if self.scenic_score.contains_key(&Direction::EAST) {
            score_vec.push(*self.scenic_score.get(&Direction::EAST).unwrap())
        }
        if self.scenic_score.contains_key(&Direction::EAST) {
            score_vec.push(*self.scenic_score.get(&Direction::EAST).unwrap())
        }

        if score_vec.len() == 0 {
            return 0;
        }

        if score_vec.len() == 1 {
            return score_vec[0]
        }

        score_vec.into_iter().reduce(|a, b| a * b).unwrap()
    }
}

fn first_star() -> usize {
    let trees = parse_input();
    let y_width: usize = trees.len();
    let x_width: usize = trees[0].len();

    let flattened_trees: Vec<Tree> = trees
        .into_iter()
        .flatten()
        .map(|tree| tree.to_owned())
        .collect::<Vec<Tree>>();

    check_all_directions(x_width, y_width, flattened_trees)
        .into_iter()
        .flatten()
        .filter(|tree| tree.visible)
        .collect::<Vec<Tree>>()
        .len()
}

fn parse_input() -> Vec<Vec<Tree>> {
    let input = fs::read_to_string(".\\src\\input.txt").unwrap();
    let input_split: Vec<&str> = input.split("\r\n").collect();
    let trees: Vec<Vec<Tree>> = input_split
        .iter()
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|num| Tree {
                    height: num.to_string().parse().unwrap(),
                    visible: false,
                    scenic_score: HashMap::new(),
                })
                .collect()
        })
        .collect();
    trees
}

fn check_all_directions(x_width: usize, y_width: usize, trees: Vec<Tree>) -> Vec<Vec<Tree>> {
    let mut visible_trees: Vec<Vec<Tree>> = Vec::new();

    for i in 0..y_width {
        let mut row: Vec<Tree> = trees[i * x_width..i * x_width + x_width]
            .iter()
            .cloned()
            .collect();
        let east = check_visible_trees(&mut row, Direction::WEST);
        let west = check_visible_trees(&mut east.iter().rev().cloned().collect(), Direction::EAST);
        visible_trees.push(west.iter().rev().cloned().collect());
    }

    let east_west_checked = visible_trees
        .into_iter()
        .flatten()
        .map(|tree| tree.to_owned())
        .collect::<Vec<Tree>>();
    let mut all_visible_trees: Vec<Vec<Tree>> = Vec::new();

    for i in 0..x_width {
        let mut row: Vec<Tree> = east_west_checked
            .iter()
            .skip(i)
            .step_by(y_width)
            .cloned()
            .collect();
        let north = check_visible_trees(&mut row, Direction::NORTH);
        let south =
            check_visible_trees(&mut north.iter().rev().cloned().collect(), Direction::SOUTH);
        all_visible_trees.push(south.iter().rev().cloned().collect());
    }

    all_visible_trees
}

fn check_visible_trees(row: &mut Vec<Tree>, dir: Direction) -> Vec<Tree> {
    let mut largest_tree = 0;
    let mut score: i32 = -2;
    for i in 0..row.len() {
        score += 1;
        if (row[i].height > largest_tree && !row[i].visible)
        || (i == 0 || i == row.len() - 1 && !row[i].visible)
        {
            row[i].set_visible(true);
            score = 0;
        }
        if row[i].height >= largest_tree {
            largest_tree = row[i].height;
            score = 0;
        };
        row[i].set_direction_score(dir.clone(), score);
    }
    row.to_vec()
}

fn second_star() -> i32 {
    let trees = parse_input();
    let y_width: usize = trees.len();
    let x_width: usize = trees[0].len();

    let flattened_trees: Vec<Tree> = trees
        .into_iter()
        .flatten()
        .map(|tree| tree.to_owned())
        .collect::<Vec<Tree>>();

    let visible_trees = check_all_directions(x_width, y_width, flattened_trees)
        .into_iter()
        .flatten()
        .map(|tree| tree.to_owned())
        .collect::<Vec<Tree>>();

    let mut result = 0;
    println!("{:?}", visible_trees);
    for tree in visible_trees {
        let score = tree.calculate_score();
        if score > result {
            result = score;
        }
    }
    result
}
