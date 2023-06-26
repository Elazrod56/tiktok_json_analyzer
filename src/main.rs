// main.rs
// main.rs is in charge of :
// - calling the lib.rs "build" function with the data as an argument -
// that will build an instance of the "Statistics" struct which contains the processed data
// - print the data to the console in a user-friendly way

mod read_file;
use core::panic;

use tiktok_json_analyzer::Statistics;

fn main() {
    println!("---------- The TikTok JSON Analyzer ----------");
    println!("This program reads the TikTok JSON data export and calculates statistics");
    println!(
        "Read the documentation at https://github.com/Elazrod56/tiktok_json_analyzer#readme\n"
    );

    let file = read_file::file_into_str("json/user_data.json");
    println!("\u{2705} File read successfully !\n");

    let data = read_file::str_into_object(file).unwrap_or_else(|err| {
        eprintln!("Error when converting file into object : {err}");
        panic!("Error when converting file into object : {err}");
    });

    let statistics = Statistics::build(data);
    println!(
        "The data of {} has been analyzed. Here are the results",
        statistics.username
    );

    println!("\n---------- LOGINS \u{1F511} ----------");
    println!(
        "In the last {} days, you logged in {} times to TikTok.",
        statistics.logins.get("Days since 1st login").unwrap(),
        statistics.logins.get("Openings").unwrap()
    ); // it's safe to unwrap because there would have been an error sooner if something was wrong
       // it's also much more concise in terms of code length
    println!(
        "You opened TikTok {} times a day on average",
        statistics.logins.get("Openings a day").unwrap()
    );

    println!("\n---------- VIDEO CONSUMPTION \u{1F4FA} ----------");
}
