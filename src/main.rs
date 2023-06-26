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
    println!(
        "In the last {} days, you have watched {} videos.",
        statistics.watched.get("Days since 1st video").unwrap(),
        statistics.watched.get("Videos watched").unwrap()
    );
    println!(
        "You watched {} videos per day on average",
        statistics.watched.get("Watched per day").unwrap()
    );

    println!("\n---------- TIME SPENT DAILY \u{1F570}----------");
    println!("Considering the fact that TikTok videos are on average 27.5 seconds long, and that you've watched {} videos a day,", statistics.watched.get("Watched per day").unwrap());
    println!("we can estimate how much time you've spent on TikTok on average.");
    println!("{}", statistics.time);
    println!("Be aware of your screen time and of the way you consume content.");

    println!("\n---------- FAVORITES \u{1F60D} ----------");
    println!(
        "You have {} favorite effects",
        statistics.favorites.get("Effects").unwrap()
    );
    println!(
        "You have {} favorite hashtags",
        statistics.favorites.get("Hashtags").unwrap()
    );
    println!(
        "You have {} favorite sounds",
        statistics.favorites.get("Sounds").unwrap()
    );
    println!(
        "You have {} favorite videos",
        statistics.favorites.get("Videos").unwrap()
    );

    println!("\n---------- LIKES \u{2764} ----------");
    println!(
        "You have liked {} videos in the past {} days",
        statistics.likes_left.get("Videos liked").unwrap(),
        statistics.likes_left.get("Days since oldest like").unwrap()
    );
    println!(
        "You have liked {} videos a day on average\n",
        statistics.likes_left.get("Likes per day").unwrap()
    );
    println!(
        "You liked around {}% of the videos you have watched",
        statistics
            .likes_left
            .get("Liked videos percentage")
            .unwrap()
    );

    println!("\n---------- COMMENTS \u{1F4AC} ----------");
    println!("You have posted {} comments", statistics.comments);

    println!("\n---------- DIRECT MESSAGES \u{2709} ----------");
    for entry in statistics.dms.iter() {
        println!("{} {} messages", entry.0, entry.1);
    }
    println!(
        "\nYou sent {} messages in total",
        statistics.dms.into_values().sum::<usize>()
    );

    println!("\n---------- AUDIENCE STATISTICS \u{1F464} ----------");

    if statistics.likes_received > 0 && statistics.videos_published == 0 {
        println!("You have received {} likes but no videos were found in the data. You must have deleted old videos", statistics.likes_received);
    } else if statistics.likes_received == 0 && statistics.videos_published == 0 {
        println!("You received no likes and published no videos");
    } else if statistics.likes_received == 0 && statistics.videos_published > 0 {
        println!(
            "You have published {} videos, but sadly no one liked them",
            statistics.videos_published
        );
    } else {
        println!(
            "You have received {} likes in {} videos",
            statistics.likes_received, statistics.videos_published
        );
        println!(
            "You got {} likes per video on average",
            (statistics.likes_received / statistics.videos_published)
        );
    }
    println!("Please note that likes received from old videos are still being counted.")
}
