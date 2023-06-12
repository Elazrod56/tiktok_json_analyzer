// Github : Elazrod56

use serde_json::Value;
use std::fs;
use std::io;
mod date_utils;

fn main() -> io::Result<()> {
    // the main function must return a Result<()> if we want to use the '?' operator on lines 18 and 19

    // rudimentary ui
    println!("-------- The TikTok JSON analyzer --------\n");
    println!("This program calculates statistics about your TikTok usage");
    println!("To run, the program needs a valid TikTok JSON data export");
    println!("Make sure that this file is located in 'json' and called 'user_data.json'");
    println!("Otherwise a 'No such file or directory' error will occur \n");

    let file = fs::read_to_string("json/user_data.json")?;
    let data = serde_json::from_str::<Value>(file.as_str())?;
    println!("\u{2705} File read successfully !");

    let username = &data["Profile"]["Profile Information"]["ProfileMap"]["userName"];
    println!("The data of {username} will be analyzed... \n");

    let day_you_asked_the_data =
        &data["Activity"]["Video Browsing History"]["VideoList"][0]["Date"];
    // the day on which you asked your TikTok data - It is useful later on
    let days_since_you_asked_the_data =
        date_utils::days_since_date(day_you_asked_the_data.as_str().unwrap()).unwrap();  
    // how many days have passed between the time you run this and the date of the data export


    // logins
    println!("\n-------- LOGINS \u{1F511} --------\n");
    let login_history = &data["Activity"]["Login History"]["LoginHistoryList"];
    let login_history_len = custom_length(login_history);

    let first_login_in_the_list = &login_history[login_history_len - 1]["Date"].to_string();
    // 'first login' refers to the earliest date which appears in the login history

    let date_of_1st_login = &first_login_in_the_list[1..20];
    // we take a slice because at the end of this string there is a 'UTC' which prevents the date_to_unix_timestamp
    // function in date_utils to correctly read the date

    let mut days_since_1st_login = date_utils::days_since_date(date_of_1st_login).unwrap();
    days_since_1st_login -= days_since_you_asked_the_data;

    println!("In the last {days_since_1st_login} days, you have opened TikTok {login_history_len} times");
    println!(
        "You opened TikTok {} times a day on average\n",
        login_history_len / days_since_1st_login // logins per day
    );

    // number of videos watched
    println!("\n-------- VIDEO CONSUMPTION \u{1F4FA} --------\n");
    let watched_videos = &data["Activity"]["Video Browsing History"]["VideoList"];
    let watched_videos_len = custom_length(watched_videos);

    // the process is almost the same as on lines 40 -> 48
    let date_of_1st_vid = &watched_videos[watched_videos_len - 1]["Date"].as_str();

    let mut days_since_1st_vid = date_utils::days_since_date(date_of_1st_vid.unwrap()).unwrap();
    // we need to unwrap twice because as_str() returns an option - also, there was no need to take a slice of
    // the string, because there was no 'UTC' at the end of it
    days_since_1st_vid -= days_since_you_asked_the_data;

    let watched_per_day = watched_videos_len / days_since_1st_vid;

    println!("In the last {days_since_1st_vid} days, you have watched {watched_videos_len} videos");
    println!("You watched an average of {watched_per_day} videos per day \n");


    // favorites
    println!("\n-------- FAVORITE EFFECTS, HASHTAGS, SOUNDS & VIDEOS \u{1F60D} --------\n");

    let favorite_effects = &data["Activity"]["Favorite Effects"]["FavoriteEffectsList"];
    let favorite_effects_len = custom_length(favorite_effects);

    let favorite_hashtags = &data["Activity"]["Favorite Hashtags"]["FavoriteHashtagList"];
    let favorite_hashtags_len = custom_length(favorite_hashtags);

    let favorite_sounds = &data["Activity"]["Favorite Sounds"]["FavoriteSoundList"];
    let favorite_sounds_len = custom_length(favorite_sounds);

    let favorite_videos = &data["Activity"]["Favorite Videos"]["FavoriteVideoList"];
    let favorite_videos_len = custom_length(favorite_videos);

    println!("You have {favorite_effects_len} favorite effects\n");
    println!("You have {favorite_hashtags_len} favorite hashtags\n");
    println!("You have {favorite_sounds_len} favorite sounds\n");
    println!("You have {favorite_videos_len} favorite videos\n");


    // likes
    println!("\n-------- LIKES \u{2764} --------\n");
    let liked_videos = &data["Activity"]["Like List"]["ItemFavoriteList"];
    let liked_videos_len = custom_length(liked_videos);

    let date_of_oldest_like = &liked_videos[liked_videos_len - 1]["Date"];
    // TikTok only keeps track of your latest 8000 likes. (at least in the data export)
    // this variable corresponds to the earliest like in the like list

    // how many days between date_of_oldest_like and the day on which the data was processed ?
    let mut days_since_oldest_like =
        date_utils::days_since_date(date_of_oldest_like.as_str().unwrap()).unwrap();

    // how many days have passed since the oldest liked video ?
    days_since_oldest_like -= days_since_you_asked_the_data;
    // 'days_since_oldest_like' = how many days there are between the time you run this and
    // 'date_since_oldest_like'. so, we substract the number of days that passed since you asked for your data.

    let likes_per_day = liked_videos_len / days_since_oldest_like;

    println!(
        "You have liked {} videos in the last {} days",
        liked_videos_len, days_since_oldest_like
    );
    println!("You have liked {likes_per_day} videos a day on average");
    println!(
        "You liked around {}% of the videos you've watched",
        ((likes_per_day as f64 / watched_per_day as f64) * 100.0) as usize
    ); // we need to calculate this using floats, otherwise the result will be rounded to 0

    if liked_videos_len == 8000 {
        println!("\nNote : The TikTok data export only keeps track of your last 8000 likes. You may have liked more.");
        println!("Don't worry, the 'likes per day' stat is still quite accurate.")
    }


    // comments
    println!("\n-------- COMMENTS \u{1F4AC} --------\n");
    let comments = &data["Comment"]["Comments"]["CommentsList"];
    let comments_len = custom_length(comments);

    println!("You have posted {comments_len} comments\n");


    // direct messages
    println!("\n-------- DIRECT MESSAGES \u{2709} --------\n");

    let chat_history = &data["Direct Messages"]["Chat History"]["ChatHistory"]
        .as_object()
        .unwrap(); // this is now a HashMap/dictionnary, with the conversation name as the key and the messages as the value
        // we can count each message in the

    let mut total_messages: usize = 0;

    for (chat_name, chat_messages) in chat_history.iter() {
        // we are iterating over each conversation

        let count = chat_messages.as_array().unwrap().len();
        // how many messages in the current conversation

        total_messages += count;

        println!("Chat with{}  {} messages", &chat_name[17..], count); // taking a slice allows to keep only the username of the person
    }
    println!("\nYou have sent a total of {total_messages} messages");
    

    // audience stats of the user
    println!("\n-------- AUDIENCE STATISTICS \u{1F464} --------\n");
    let videos = &data["Video"]["Videos"]["VideoList"];
    let videos_len = custom_length(videos);

    let likes: &Option<&str> = &data["Profile"]["Profile Information"]["ProfileMap"]["likesReceived"].as_str();
    // the following block parses the likes as an integer : but if the user received no likes, the value
    // is "None" (as a string). so we need to associate "None" to 0
    let likes_u64 = if let Some(likes_str) = *likes {
        if likes_str == "None" {
            0
        } else {
            likes_str.parse::<u64>().unwrap()
        }
    } else {
        0
    };

    if videos_len != 0 && likes_u64 != 0 {
        println!(
            "You have received {} likes and you have posted {} videos",
            likes_u64, videos_len
        );
        println!(
            "Meaning that you have received an average of {} likes per video",
            likes_u64 / videos_len as u64
        );
        println!("\nNote : Likes from deleted videos are still counted, so this statistic might not be 100% accurate");
    } else if likes_u64 != 0 && videos_len == 0 {
        println!("You have received {likes_u64} likes but no videos were found in the data");
        println!("It is likely that you have deleted old videos");
    } else {
        println!("No videos or received likes were found");
    }

    Ok(())
}

fn custom_length(input_value: &Value) -> usize {
    // calculates how many elements there are in a JSON category
    input_value.as_array().map(|a| a.len()).unwrap_or(0)
}

// TESTS

#[test]
fn file_is_readable() {
    // this function will check if the file is placed in the correct directory and is a JSON file.
    // otherwise it will panic

    let file = fs::read_to_string("json/user_data.json");
    match file {
        Ok(_) => {
            println!("File detected and read successfully!");
        }
        Err(error) => {
            println!("Error reading file: {:?}", error);
            panic!("File was not found or there was an error reading it. Make sure it is located in 'json/user_data.json'")
        }
    }
}

#[test]
fn file_is_valid() {
    // this test is supposed to run after 'file_is_readable' to make sure that the file is TikTok data
    // obviously it is not perfect since you could just write ["Activity"] and ["Favorite Effects"]
    // in a blank JSON file, but seriously, who would do this ?

    let file = fs::read_to_string("json/user_data.json");
    let data = serde_json::from_str::<Value>(file.unwrap().as_str());

    // when given tags that do not exist, we get "Null". So we verify that some tags are reachable
    assert_ne!(
        &data.unwrap()["Activity"]["Favorite Effects"],
        "Null",
        "The file doesn't seem to be TikTok data"
    );
}
