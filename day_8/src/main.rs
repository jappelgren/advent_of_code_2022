// https://adventofcode.com/2022/day/8
use std::fs;

// Check highest so far tree in row or column and compare to that
// Skip trees that are already visible from any orientation.

fn main() {
    println!("The result for the first star is: {:?}", first_star());
    println!("The result for the second star is: {:?}", second_star());
}

#[derive(Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn set_visible(&mut self, visibility: bool) {
        self.visible = visibility;
    }
}

fn first_star() -> usize {
    let input = input_to_string();
    let input_split: Vec<&str> = input.split("\r\n").collect();
    let trees: Vec<Vec<Tree>> = input_split
        .iter()
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|num| Tree {
                    height: num.to_string().parse().unwrap(),
                    visible: false,
                })
                .collect()
        })
        .collect();
    let y_width: usize = trees.len();
    let x_width: usize = trees[0].len();

    let flattened_trees: Vec<Tree> = trees
        .into_iter()
        .flatten()
        .map(|tree| tree.to_owned())
        .collect::<Vec<Tree>>();

    return check_all_directions(x_width, y_width, flattened_trees);
}

fn check_all_directions(x_width: usize, y_width: usize, trees: Vec<Tree>) -> usize {
    let mut visible_trees: Vec<Vec<Tree>> = Vec::new();

    for i in 0..y_width {
        let mut row: Vec<Tree> = trees[i * x_width..i * x_width + x_width]
            .iter()
            .copied()
            .collect();
        let east = check_visible_trees(&mut row);
        let west = check_visible_trees(&mut east.iter().rev().copied().collect());
        visible_trees.push(west.iter().rev().copied().collect());
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
            .copied()
            .collect();
        let north = check_visible_trees(&mut row);
        let south = check_visible_trees(&mut north.iter().rev().copied().collect());
        all_visible_trees.push(south.iter().rev().copied().collect());
    }

    let result = all_visible_trees
        .into_iter()
        .flatten()
        .filter(|tree| tree.visible)
        .collect::<Vec<Tree>>()
        .len();

    result
}

fn check_visible_trees(row: &mut Vec<Tree>) -> Vec<Tree> {
    let mut largest_tree = 0;
    for i in 0..row.len() {
        if (row[i].height > largest_tree && !row[i].visible)
            || (i == 0 || i == row.len() - 1 && !row[i].visible)
        {
            row[i].set_visible(true);
        }
        if row[i].height > largest_tree {
            largest_tree = row[i].height
        };
    }
    row.to_vec()
}

fn second_star() {}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\example_input.txt").unwrap()
}
