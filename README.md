# The TikTok JSON analyzer

This is a program to analyze your TikTok data and calculate these statistics :

- Number of logins (in the last 6 months) and logins per day
- Number of videos watched (in the last 6 months) and watched videos per day
- Number of favorite effects, hashtags, sounds and videos
- Statistics about the likes you left
- Number of comments posted
- Number of DMs sent
- Your account's audience stats

I pretty much ran out of ideas for new statistics, but if you have one, share it!

## How to get your TikTok data?

In your TikTok app, Head over to Settings > Account > Get my data and ask for JSON.

You should receive a zipped folder containing your data export in the following days

## How to use the program?

You have two options :

1. Compile it yourself :

    1.1 : Make sure rustc and cargo are installed, check out [this link](https://rust-lang.org) for further information.  
    1.2 : In your terminal, run `git clone https://github.com/Elazrod56/tiktok_json_analyzer.git`  
    1.3 : A new folder called `tiktok_json_analyser` will be created, open it in your terminal.  
    1.4 : Place your TikTok data in `json/` and name it `user_data.json`. Normally it is already named as such  
    1.5 : Finally, in `tiktok_json_analyzer`, run `cargo run`. 

2. Use the released binaries :

	2.1 : Go into 'releases' and select the latest version  
	2.2 : Download the executable for your version  
	2.3 : Place it in an individual folder  
	2.4 : Make sure that in the same folder there is a `json` folder containing `user_data.json`  
	2.5 : Open your terminal  
	2.6 : On Linux, run `./linux_x64`. *(Note : you might need to make it executable by typing `chmod +x linux_x64`)*  
	2.7 : On Windows, run `.\windows_x64.exe`  

## What if you get an error?

If you get a "no such file or directory error", make sure that "user_data.json" is in the "json" directory.

If you get another error, feel free to open an Issue (or a PR if you have a solution)

## For contributors

If you want to contribute to this project, you can contact me on Discord (elazrod#7500) or open a PR. I'll be happy to collaborate with you.
