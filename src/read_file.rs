// read_file.rs
// read_file.rs is in charge of :
// - reading the user_data.json file and transform it into a String
// - transforming the String into a serde_json::Value object
// the program panics if there is any kind of error (such as file not found - the most likely to occur)

use serde_json::{Error, Value};
use std::{fs, process};

// these need to be public for the main function to access them
pub fn file_into_str(file_path: &str) -> String {
    let file = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("\u{274C} ERROR: Could not read file - {:?}", err);
        process::exit(1);
    });

    file
}

pub fn str_into_object(file_string: String) -> Result<Value, Error> {
    let data = serde_json::from_str::<Value>(file_string.as_str());

    data
}
