# The TikTok JSON analyzer

This is a simple program to analyze your TikTok data and provide the following info :

- Amount of times you opened the app (in the last 6 months)
- The amount of videos you watched (in the last 6 months)
- How many videos and sounds you have as favorite
- Statistics about the likes you left on videos, such as the likes/day stat
- How many comments have you published
- Your account's stats (likes received and videos)

This project is still in early development. I will add new features in the future


## How to get the data?

In your TikTok app, Head over to Settings > Account > Get my data and ask for JSON.

You should get it in the following 4 days.


## How to use it?

1. Make sure rustc and cargo are installed, check out [this link](https://rust-lang.org) for further information.

2. Then, open your terminal and type `git clone https://github.com/Elazrod56/tiktok_json_analyzer.git`

3. A new folder called 'tiktok_json_analyser' will be created, open it in your terminal.

4. Place your file in `json/user_data.json`

5. Finally, fire up your terminal and run `cargo run`. The code will compile and print the stats


## I get an error

If you get a "no such file or directory error", make sure that "user_data.json" is in the "json" directory.

If you get another error, feel free to open an Issue (or a PR if you have a solution)

## For contributors

If you want to contribute to this project, you can contact me on Discord (Elazrod#7500) or open a PR. I'll be happy to collaborate with you.
