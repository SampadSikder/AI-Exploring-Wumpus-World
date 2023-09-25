use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use super::logic::CellKnowledge;



// Function to save the knowledge_base vector to a JSON file
pub fn save_knowledge_base(filename: &str, knowledge_base: &[Vec<CellKnowledge>]) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    // Serialize the knowledge_base vector to JSON and write it to the file
    let serialized = serde_json::to_string(&knowledge_base)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub(crate) fn load_knowledge_base(filename: &str) -> Result<Vec<Vec<CellKnowledge>>, Box<dyn Error>> {
    // Open the file for reading
    let mut file = File::open(filename)?;

    // Read the contents of the file into a string
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    // Deserialize the JSON string into a vector of vectors of CellKnowledge
    let loaded_knowledge_base: Vec<Vec<CellKnowledge>> = serde_json::from_str(&json)?;

    Ok(loaded_knowledge_base)
}

pub(crate) fn save_coordinates_to_file(coordinates: &[(i32, i32)], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, coordinates)?;
    Ok(())
}

pub(crate) fn load_coordinates_from_file(filename: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let coordinates: Vec<(i32, i32)> = serde_json::from_reader(reader)?;
    Ok(coordinates)
}

/* 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage
    let filename = "knowledge_base.json";

    // Save the knowledge_base vector to a file
    save_knowledge_base(filename, &knowledge_base)?;

    // Load the knowledge_base vector from a file
    knowledge_base = load_knowledge_base(filename)?;

    // Now knowledge_base is populated with the data from the file

    Ok(())
}
*/