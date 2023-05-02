# The TikTok JSON analyzer

This is a simple program to analyze your TikTok data and provide the following info :

- Amount of times you opened the app (last 6 months)
- The number of videos you watched (last 6 months)
- How many videos and sounds do you have as favorite
- How long did it take you to like 8000 videos?
- How many comments have you published
- Your account's stats (likes received and videos)

This project is still in (very) early development. I will implement new features in the future

## How to get the data?

Head over to Settings > Account > Get my data and ask for JSON.

You should get it in the following 4 days.

## How to use it?

Place your file in `json/user_data.json`, then fire up your terminal and run `cargo run`. The code will compile and analyze your data.

Note: You need to have rustc and cargo installed, check out [this link](https://rust-lang.org) for more information.

## I get an error

If you get a "no such file or directory error", make sure that "user_data.json" is in the "json" directory.

If you get another error, feel free to open an Issue (or a PR if you have a solution)

## For contributors

If you want to contribute to this project, you can contact me on Discord (Elazrod#7500) or open a PR. I'll be happy to collaborate with you.
