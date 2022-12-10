use std::{
    fs,
    collections::HashMap
};

use regex::Regex;

fn main() {
    let input: String = input_to_string();
    let re = Regex::new(r"\$ ").unwrap();
    let instructions: Vec<String> = input
        .split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|instruction| re.replace_all(instruction, "").to_string())
        .filter(|instruction| instruction != &format!("ls"))
        .collect();

    println!("The result for the first star is: {:?}", first_star(create_file_tree(&instructions)));
    println!("The result for the second star is: {:?}", second_star(create_file_tree(&instructions)));
}

#[derive(Debug)]
struct FileTree {
    directories: HashMap<String, Directory>,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>
    
}

impl Directory {
    fn create_file(&mut self, name: String, size: i32) {
        let file: File = File{ name, size};
        self.files.insert(file.name.to_string(), file);
    }
    
    fn create_dir(&mut self, name: String) {
        let directory = Directory{ name, files: HashMap::new(), directories: HashMap::new() }; 
        self.directories.insert(directory.name.to_string(), directory);
    }

    fn get_files_size (&self) -> i32 {
        self.files.values().map(|file| file.size).sum()
    }

    fn get_dir_total_size (&self) -> i32 {
        let mut result = self.get_files_size();

        for dir in get_all_directories_in(self) {
            result += dir.get_files_size()
        }
        
        result
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

fn create_file_tree(instructions: &Vec<String>) -> FileTree {
    let mut file_tree: FileTree = FileTree {
        directories: HashMap::new()
    };
    file_tree.directories.insert(format!("root"), Directory{name: format!("root"), directories: HashMap::new(), files: HashMap::new()});
    let mut cur_dir = vec!(format!("root"));
    
    for instruction in instructions {
        
        let int_vec: Vec<&str> = instruction.split(" ").collect();

        match int_vec[0] {
            "cd" => change_dir(&mut cur_dir, int_vec[1].to_string()),
            "dir" => get_current_directory(&mut file_tree, &cur_dir).create_dir(int_vec[1].to_string()),
            i if i.chars().any(|x| x.is_numeric()) => get_current_directory(&mut file_tree, &cur_dir).create_file(int_vec[1].to_string(), int_vec[0].parse().unwrap()),
            _ => println!("Didn't expect input to contain {}", instruction),
        }
    }
    file_tree
}

fn change_dir(dir_vec: &mut Vec<String>, change_string: String) {
    if change_string == format!("..") {
        dir_vec.pop();
    } else {
        dir_vec.push(change_string);
    }
}

fn get_current_directory <'a> (file_tree: &'a mut FileTree, pwd: &Vec<String>) -> &'a mut Directory {

    let mut curdir = file_tree.directories.get_mut(pwd.first().unwrap()).unwrap();

    for directory in pwd.iter().skip(1) {
        curdir = curdir.directories.get_mut(directory).unwrap()
    }

    curdir
}

fn get_all_directories_in (directory: &Directory) -> Vec<&Directory> {

    let mut directories: Vec<&Directory> = vec!();

    for directory in directory.directories.values() {
        directories.push(directory);
        directories.append(&mut get_all_directories_in(directory))
    }

    directories
}

fn first_star(file_tree: FileTree) -> i32 {
     let mut all_dirs = vec!(file_tree.directories.get("root").unwrap());
     all_dirs.append(&mut get_all_directories_in(&all_dirs[0]));

     all_dirs
     .iter()
     .map(|dir| dir.get_dir_total_size())
     .filter(|size| *size <= 100000)
     .reduce(|a,b| a + b)
     .unwrap()
}

fn second_star(file_tree: FileTree) -> i32 {
    let space_needed = 70000000 - file_tree.directories.get("root").unwrap().get_dir_total_size();

    let mut all_dirs = vec!(file_tree.directories.get("root").unwrap());
    all_dirs.append(&mut get_all_directories_in(&all_dirs[0]));

    let mut sizes: Vec<i32> = all_dirs
    .iter()
    .map(|dir| dir.get_dir_total_size())
    .filter(|size| *size + space_needed >= 30000000).collect();

    sizes.sort_unstable();

    *sizes.first().unwrap()
}

fn input_to_string() -> String {
    fs::read_to_string(".\\src\\raw_input.txt").unwrap()
}
