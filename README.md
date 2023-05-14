# The TikTok JSON analyzer

This is a program to analyze your TikTok data and calculate these statistics :

- Number of logins (in the last 6 months) and logins per day
- Number of videos watched (in the last 6 months) and watched videos per day
- Number of favorite effects, hashtags, sounds and videos
- Statistics about the likes you left
- Number of comments posted
- Your account's audience stats

This project is still in development.  
I plan to deploy the app on the web in the future so stay tuned

## How to get your TikTok data?

In your TikTok app, Head over to Settings > Account > Get my data and ask for JSON.

You should receive a zipped folder containing your data export in the following days

## How to use the program?

1. Make sure rustc and cargo are installed, check out [this link](https://rust-lang.org) for further information.

2. Open your terminal and type `git clone https://github.com/Elazrod56/tiktok_json_analyzer.git`

3. A new folder called 'tiktok_json_analyser' will be created, open it in your terminal.

4. Place your TikTok data in `json/` and name it `user_data.json`. Normally it is already named as such

5. Finally, fire up your terminal and run `cargo run`. The code will compile and print the stats

## What if you get an error?

If you get a "no such file or directory error", make sure that "user_data.json" is in the "json" directory.

If you get another error, feel free to open an Issue (or a PR if you have a solution)

## For contributors

If you want to contribute to this project, you can contact me on Discord (Elazrod#7500) or open a PR. I'll be happy to collaborate with you.
