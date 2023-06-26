// lib.rs
// lib.rs is in charge of :
// - Turning the arguments given by main.rs (= the user's TikTok data) into an instance of the "Statistics" struct
// The library processes all of this information and nicely puts it into an instance of the struct, which is then
// returned to main.rs

use serde_json::Value;
use std::collections::HashMap;
mod date_utils;

pub struct Statistics {
    pub username: String,
    pub logins: HashMap<String, usize>,
    pub watched: HashMap<String, usize>,
    pub time: String,
    pub favorites: HashMap<String, usize>,
    pub likes_left: HashMap<String, usize>,
    pub comments: usize,
    pub dms: HashMap<String, usize>,
    pub likes_received: usize,
    pub videos_published: usize,
}

impl Statistics {
    // -> Result<Statistics, &'static str>
    pub fn build(data: Value) -> Statistics {
        let username = String::from(
            &data["Profile"]["Profile Information"]["ProfileMap"]["userName"].to_string(),
        );

        let day_of_data_request =
            &data["Activity"]["Video Browsing History"]["VideoList"][0]["Date"].as_str();
        let days_since_data_request =
            date_utils::days_since_date(day_of_data_request.unwrap()).unwrap();

        let watched_per_day = read_videos(days_since_data_request, &data)
            .get("Watched per day")
            .unwrap_or(&0usize)
            .to_owned();

        let statistics = Statistics {
            username,
            logins: read_logins(days_since_data_request, &data),
            watched: read_videos(days_since_data_request, &data),
            time: daily_time(watched_per_day),
            favorites: favorites(&data),
            likes_left: likes(days_since_data_request, &data),
            comments: comments(&data),
            dms: private_messages(&data),
            likes_received: audience_stats(&data)
                .get("Likes received")
                .unwrap_or(&0usize)
                .to_owned(),
            videos_published: audience_stats(&data)
                .get("Videos published")
                .unwrap_or(&0usize)
                .to_owned(),
        };

        statistics
    }
}

// FUNCTIONS WHICH READ DATA
fn read_logins(days_since_data_request: usize, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let login_history = &data["Activity"]["Login History"]["LoginHistoryList"];
    let login_history_len = value_length(login_history);

    let date_of_1st_login = &login_history[login_history_len - 1]["Date"].to_string()[1..20];
    // we take a slice because at the end of this string there is a 'UTC' which prevents the date_to_unix_timestamp
    // function in date_utils to correctly read the date

    let mut days_since_1st_login = date_utils::days_since_date(date_of_1st_login).unwrap();
    days_since_1st_login -= days_since_data_request;

    result.insert(String::from("Days since 1st login"), days_since_1st_login);
    result.insert(String::from("Openings"), login_history_len);
    result.insert(
        String::from("Openings a day"),
        login_history_len / days_since_1st_login,
    );

    result
}

fn read_videos(days_since_data_request: usize, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    let watched_videos = &data["Activity"]["Video Browsing History"]["VideoList"];
    let watched_videos_len = value_length(watched_videos);

    let date_of_1st_vid = &watched_videos[watched_videos_len - 1]["Date"].as_str();

    let mut days_since_1st_vid = date_utils::days_since_date(date_of_1st_vid.unwrap()).unwrap();
    // we need to unwrap twice because as_str() returns an option - also, there was no need to take a slice of
    // the string, because there was no 'UTC' at the end of it
    days_since_1st_vid -= days_since_data_request;

    let watched_per_day = watched_videos_len / days_since_1st_vid;

    result.insert(String::from("Days since 1st video"), days_since_1st_vid);
    result.insert(String::from("Videos watched"), watched_videos_len);
    result.insert(String::from("Watched per day"), watched_per_day);

    result
}

fn daily_time(watched_per_day: usize) -> String {
    let total_time_in_minutes = (watched_per_day as f32 * 27.5) as usize / 60;
    // We are converting types for precision and readability purposes. (one after the other)
    // Originally the time is in seconds, but we transform it into minutes for simplicity (that's the / 60).
    let hours = total_time_in_minutes / 60;
    let minutes = total_time_in_minutes % 60;

    let result = format!(
        "You've spent on average {} hours and {} minutes on TikTok every day",
        hours, minutes
    );
    result
}

fn favorites(data: &Value) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    let effects_len = value_length(&data["Activity"]["Favorite Effects"]["FavoriteEffectsList"]);

    let hashtags_len = value_length(&data["Activity"]["Favorite Hashtags"]["FavoriteHashtagList"]);

    let sounds_len = value_length(&data["Activity"]["Favorite Sounds"]["FavoriteSoundList"]);

    let videos_len = value_length(&data["Activity"]["Favorite Videos"]["FavoriteVideoList"]);

    result.insert(String::from("Effects"), effects_len);
    result.insert(String::from("Hashtags"), hashtags_len);
    result.insert(String::from("Sounds"), sounds_len);
    result.insert(String::from("Videos"), videos_len);

    result
}

fn likes(days_since_data_request: usize, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let watched_per_day = read_videos(days_since_data_request, &data)
        .get("Watched per day")
        .unwrap_or(&0usize)
        .to_owned();
    // this is needed to calculate the like percentage

    let liked_videos = &data["Activity"]["Like List"]["ItemFavoriteList"];
    let liked_videos_len = value_length(liked_videos);

    let date_of_oldest_like = &liked_videos[liked_videos_len - 1]["Date"];
    // TikTok only keeps track of your latest 8000 likes. (at least in the data export)
    // this variable corresponds to the earliest like in the like list

    // how many days between date_of_oldest_like and the day on which the data was processed ?
    let mut days_since_oldest_like =
        date_utils::days_since_date(date_of_oldest_like.as_str().unwrap()).unwrap();

    // how many days have passed since the oldest liked video ?
    days_since_oldest_like -= days_since_data_request;
    // 'days_since_oldest_like' = how many days there are between the time you run this and
    // 'date_since_oldest_like'. so, we substract the number of days that passed since you asked for your data.

    let likes_per_day = liked_videos_len / days_since_oldest_like;

    result.insert(String::from("Videos liked"), liked_videos_len);
    result.insert(
        String::from("Days since oldest like"),
        days_since_oldest_like,
    );
    result.insert(String::from("Likes per day"), likes_per_day);
    result.insert(
        String::from("Liked videos percentage"),
        ((likes_per_day as f64 / watched_per_day as f64) * 100.0) as usize,
    ); // we need to calculate this using floats, otherwise the result will be rounded to 0

    result
}

fn comments(data: &Value) -> usize {
    value_length(&data["Comment"]["Comments"]["CommentsList"])
}

fn private_messages(data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let chat_history = data["Direct Messages"]["Chat History"]["ChatHistory"]
        .as_object()
        .unwrap(); // this is now a HashMap/dictionnary, with the conversation name as the key and the messages as the value

    for (chat_name, chat_messages) in chat_history.iter() {
        // we are iterating over each conversation

        let count = chat_messages.as_array().unwrap().len();
        // how many messages in the current conversation

        result.insert(format!("Chat with{}", &chat_name[17..]), count); // taking a slice allows to keep only the username of the person
    }

    result
}

fn audience_stats(data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let videos_published = value_length(&data["Video"]["Videos"]["VideoList"]);

    let likes_received: &Option<&str> =
        &data["Profile"]["Profile Information"]["ProfileMap"]["likesReceived"].as_str();
    // the following block parses the likes as an integer : but if the user received no likes, the value
    // is "None" (as a string). so we need to associate "None" to 0
    let likes_u64 = if let Some(likes_str) = *likes_received {
        if likes_str == "None" {
            0
        } else {
            likes_str.parse::<u64>().unwrap()
        }
    } else {
        0
    };

    result.insert(String::from("Videos published"), videos_published);
    result.insert(String::from("Likes received"), likes_u64 as usize);

    result
}

// Calculates how many elements there are in a JSON category
fn value_length(input_value: &Value) -> usize {
    input_value.as_array().map(|a| a.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use core::panic;

    mod tests_read_file;

    // These functions test what happens in different cases by using file that I made.
    // There are other functions in main.rs which do the same thing, but with the file json/user_data.json
    // in order to check if the file is readable and valid

    #[test]
    #[should_panic(expected = "Error reading the file")]
    fn file_not_found() {
        // this function tests what happens when the file isn't found
        let file_path = "/path/to/absolutely/no/file.txt";
        tests_read_file::file_into_str(file_path);
    }

    #[test]
    fn file_found_but_not_valid() {
        // This function tests what happens when the file is found but isn't valid
        let file_path = "src/tests/not_valid_file.json";
        let file_content = tests_read_file::file_into_str(file_path);
        let data = tests_read_file::str_into_object(file_content).unwrap_or_else(|err| {
            eprintln!("Error while trying to read string: {err}");
            panic!("Error while trying to read string: {err}")
        });
        let username = &data["Profile"]["Profile Information"]["ProfileMap"]["userName"];

        // To check the validity of the file, we are checking if we can find the userName value.
        // If we can't, then the file is not valid
        // Obviously we could just copy-paste the "Profile" section of regular data into the file and it would work...
        // But then we can say that you have tried really hard to make the program fail!
        assert_eq!(username.as_str(), None);
        // In this case, we're checking if we can access the "userName" value, which doesn' exist in not_valid_file.json
        // When trying to access a value that does not exist, serde_json returns a None value.
    }

    #[test]
    fn all_good() {
        // This function tests if the file is readable and is valid (= userName value can be read)
        // The file given to analyze is not the real TikTok data file, but it contains the username,
        // in order to pass the test.
        let file_path = "src/tests/partially_valid_file.json";
        let file_content = tests_read_file::file_into_str(file_path);
        let data = tests_read_file::str_into_object(file_content).unwrap_or_else(|err| {
            eprintln!("Error while trying to read string: {err}");
            panic!("Error while trying to read string: {err}")
        });
        let username = &data["Profile"]["Profile Information"]["ProfileMap"]["userName"];

        assert_eq!(username.as_str(), Some("john.doe"));
        // Same as for function file_found_but_not_valid, but here the value exists
    }
}
