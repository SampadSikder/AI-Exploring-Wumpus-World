use std::collections::HashSet;

use super::save_kb::{load_coordinates_from_file, save_coordinates_to_file};

pub fn add_path(x: i32,y: i32){
    let mut result = load_coordinates_from_file(&String::from("path.txt"));

    match result {
        Ok(mut paths)=>{
            paths.push((x,y));
            save_coordinates_to_file(&paths, &String::from("path.txt"));
        }
        Err(error)=>{
            print!("{:?}",error);
        }
    }

}

fn has_loop(path: &Vec<(i32,i32)>) -> bool {
    let mut visited_nodes = HashSet::new();

    for node in path {
        if !visited_nodes.insert(node) {
            return true;
        }
    }

    // No loop detected
    false
}

pub fn detect_loop()->bool{
    let mut result = load_coordinates_from_file(&String::from("path.txt"));

    match result {
        Ok(mut paths)=>{
            return has_loop(&paths);
        }
        Err(error)=>{
            print!("{:?}",error);
            return false;
        }
    }
}