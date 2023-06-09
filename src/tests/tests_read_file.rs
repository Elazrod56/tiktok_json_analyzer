// tests_read_file.rs - COPY OF src/read_file.rs
// This is the exact same file as the one in src/read_file.rs
// Why ? Because this way, the test functions in src/lib.rs can access these functions

// read_file.rs normally is in charge of :
// - reading the user_data.json file and transform it into a String
// - transforming the String into a serde_json::Value object
// the program is exited if there is any kind of error (such as file not found - the most likely to occur)
// NOT REALLY, THESE FUNCTIONS ARE ONLY USED BY TEST FUNCTIONS

use serde_json::{Error, Value};
use std::fs;

// these need to be public for the test functions to access them
pub fn file_into_str(file_path: &str) -> String {
    let file = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading the file: {:?}", err);
        panic!("Error reading the file: {:?}", err);
    });

    file
}

pub fn str_into_object(file_string: String) -> Result<Value, Error> {
    let data = serde_json::from_str::<Value>(file_string.as_str());

    data
}
