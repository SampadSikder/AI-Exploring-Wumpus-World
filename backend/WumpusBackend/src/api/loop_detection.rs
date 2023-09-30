use std::collections::{HashSet, HashMap};

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

fn has_loop(path: &Vec<(i32,i32)>, min_occurrences: usize) -> bool {
    if path.is_empty() {
        return false; // Empty path cannot have loops
    }

    let mut visited_nodes = HashMap::new();

    for (index, node) in path.iter().enumerate() {
        let count = visited_nodes.entry(node).or_insert(0);
        *count += 1;

        // If the node has been visited at least min_occurrences times
        if *count >= min_occurrences {

            // Clear Path File to ensure that same loop is not detected everytime.
            let _ = save_coordinates_to_file(&[], "path.txt");

            return true; // Loop detected
        }

        visited_nodes.insert(node, index);
    }

    // No loop detected
    false
}

pub fn detect_loop()->bool{
    let mut result = load_coordinates_from_file(&String::from("path.txt"));

    match result {
        Ok(mut paths)=>{
            return has_loop(&paths, 3);
        }
        Err(error)=>{
            print!("{:?}",error);
            return false;
        }
    }
}