# The TikTok JSON analyzer

A Rust program to analyze TikTok data and calculate some statistics :

- Logins / App openings
- Videos watched
- Time spent on TikTok per day
- Favorite effects, hashtags, sounds and videos
- Statistics about the likes you left
- Comments posted
- DMs Stats
- Your account's audience stats

If you have an idea for a new stat, share it!

## How to get your TikTok data?

In your TikTok app, Head over to Settings > Account > Get my data and ask for JSON.

When the data is ready, download it. It is a zipped folder which contains the file user_data.json.

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
	2.6 : On Linux, run `./tiktok_json_analyzer_linux-x64`. *(note : you might need to make it executable by typing `chmod +x tiktok_json_analyzer_linux-x64`)*  
	2.7 : On Windows, run `.\tiktok_json_analyzer_windows-x64.exe` (make sure to run it in a terminal window! double-clicking the file will open and close a window too rapidly for you to read the output.)  

## What if you get an error?

If you get a "no such file or directory error", make sure that "user_data.json" is in the "json" directory.

If you get another error, feel free to open an Issue (or a PR if you have a solution)

## Binaries

- The Linux binary was compiled on Arch Linux 64-bit with Rust 1.70.0. It should work on any 64-bit Linux distro.
- The Windows binary was cross-compiled on Arch 64-bit too, with Rust 1.70.0. It should run on both Windows 10 and 11.

If the binaries do not run on your machine, consider compiling the code yourself.

## For contributors

If you want to contribute to this project, you can contact me on Discord (elazrod#7500) or open a PR. I'll be happy to collaborate with you.

## Why did I make this project?

At first, I wanted to see what TikTok data looked like, so I asked a friend to give me his data.
Then I said to myself "Hey, I could code something to calculate stats over this", and I did it.  
  
At first, I made the project in Python for simplicity, but since I was learning Rust, I said to myself that
I would do it in Rust to practice. That's why I chose an overkill language for such a simple project.

## Future direction

This project may evolve in the future, I have two main directions for it :
- A GUI version : click, select your file and get the results
- A web app version : upload your file and get the results

Basically, I intent to make the Analyzer more accessible to the average non-IT person to make these people more aware of the way they consume content.
