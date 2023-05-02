// Github : Elazrod56

use serde_json::Value;
use std::fs;
use std::io;

mod date_utils;

fn main() -> io::Result<()> {
    // The main function must return a Result<()> if we want to use the '?' operator on lines 14 and 15

    println!("-------- THE TIKTOK JSON ANALYZER --------");
    println!("This program calculates some statistics using your TikTok JSON data export");
    println!("Please make sure the JSON file is located in json/user_data.json \n");

    let file = fs::read_to_string("json/user_data.json")?;
    let data = serde_json::from_str::<Value>(file.as_str())?;

    let day_you_asked_the_data =
        &data["Activity"]["Video Browsing History"]["VideoList"][0]["Date"];
    // The day on which you asked your TikTok data - It is useful later on
    let days_since_you_asked_the_data =
        date_utils::days_since_date(day_you_asked_the_data.as_str().unwrap()).unwrap();
    // How many days have passed since you asked for your data

    let username = &data["Profile"]["Profile Information"]["ProfileMap"]["userName"];
    println!(
        "File detected \u{2705}
        \nThe data of {username} will be analyzed... \n"
    );

    // Amount of logins
    println!("\n-------- LOGINS \u{2386} --------\n");
    let login_history = &data["Activity"]["Login History"]["LoginHistoryList"];
    let login_history_len = login_history
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    let first_login_in_the_list = &login_history[login_history_len - 1]["Date"].to_string(); // Note : 'first login'
                                                                                             // refers to the earliest date which appears in the login history. (the one that's the most in the past)

    let date_of_1st_login = &first_login_in_the_list[1..20];
    // We take only a slice because at the end of the string there is a 'UTC' which prevents the date_to_unix_timestamp
    // function in date_utils to correctly interpret the date

    let mut days_since_1st_login = date_utils::days_since_date(date_of_1st_login).unwrap();
    days_since_1st_login -= days_since_you_asked_the_data;

    println!("In the last {days_since_1st_login} days, you connected {login_history_len} times to Tiktok");
    println!(
        "You launched TikTok {} times a day on average\n",
        login_history_len / days_since_1st_login as usize
    );

    // Amount of videos watched
    let watched_videos = &data["Activity"]["Video Browsing History"]["VideoList"];
    let watched_videos_len = watched_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    // The process is almost the same as on lines 38 -> 52
    let date_of_1st_vid_in_the_list = &watched_videos[watched_videos_len - 1]["Date"].as_str();

    let mut days_since_1st_vid =
        date_utils::days_since_date(date_of_1st_vid_in_the_list.unwrap()).unwrap();
    // This time we need to unwrap twice because as_str() returns an option. We did not get type &str at first because we had
    // no need to get a slice this time since there were no annoying 'UTC'
    days_since_1st_vid -= days_since_you_asked_the_data;

    println!("In the last {days_since_1st_vid} days, you have watched {watched_videos_len} videos");
    println!(
        "You watched around {} videos a day on average\n",
        watched_videos_len / days_since_1st_vid as usize
    );

    // Favorites
    println!("\n-------- FAVORITE SOUNDS & VIDEOS \u{1F4FA} --------\n");
    let favorite_sounds = &data["Activity"]["Favorite Sounds"]["FavoriteSoundList"];
    let favorite_sounds_len = favorite_sounds
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("You have {favorite_sounds_len} favorite sounds\n");

    let favorite_videos = &data["Activity"]["Favorite Videos"]["FavoriteVideoList"];
    let favorite_videos_len = favorite_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("You have {favorite_videos_len} favorite videos\n");

    // Likes
    println!("\n-------- LIKES \u{2764} --------\n");
    let liked_videos = &data["Activity"]["Like List"]["ItemFavoriteList"];
    let liked_videos_len = liked_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    let date_of_8000th_liked_vid = &liked_videos[liked_videos_len - 1]["Date"];
    // TikTok only keeps track of your latest 8000 likes. (at least in the data export)
    println!("You liked 8000 videos since {date_of_8000th_liked_vid}\n");

    // How many days between date_of_8000th_liked_vid and the day on which the data was processed ?
    let mut days_since_8000th_like =
        date_utils::days_since_date(date_of_8000th_liked_vid.as_str().unwrap()).unwrap();
    // How many days have passed since your 8000th liked video ?
    days_since_8000th_like = days_since_8000th_like - days_since_you_asked_the_data;
    // The variable 'days_since_8000th_like' contains how many days there are between the time you run this and
    // 'date_since_8000th_like'. So, we substract the number of days that passed since you asked for your data.
    // By doing so, we get the number of days that passed since your 8000th like. Hope this was clear enough ;)

    println!(
        "It took you {} days to like 8000 videos",
        days_since_8000th_like
    );
    println!(
        "Meaning that you have liked {} videos a day on average",
        8000 / days_since_8000th_like
    );

    // Comments
    println!("\n-------- COMMENTS \u{1F4AC} --------\n");
    let comments = &data["Comment"]["Comments"]["CommentsList"];
    let comments_len = comments.as_array().map(|array| array.len()).unwrap_or(0);

    println!("You published {comments_len} comments\n");

    // Videos & likes
    println!("\n-------- YOUR ACCOUNT'S STATS \u{1F464} --------\n");
    let videos = &data["Video"]["Videos"]["VideoList"];
    let videos_len = videos.as_array().map(|array| array.len()).unwrap_or(0);

    let likes = &data["Profile"]["Profile Information"]["ProfileMap"]["likesReceived"];
    println!("You received {likes} likes and you posted {videos_len} videos");

    Ok(())
}
