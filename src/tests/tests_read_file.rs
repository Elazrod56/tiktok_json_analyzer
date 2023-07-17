// these functions are just test functions. they are only there for test-driven development

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
