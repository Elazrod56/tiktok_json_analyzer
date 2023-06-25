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

When the data is ready, download it. It is a zipped folder which contains the file `user_data.json` (your data).  
I will refer to this file as "TikTok data".

## How to use the program?

You have two options :

### Compiling it yourself :

    1. Make sure rustc and cargo are installed - Check out [this link](https://rust-lang.org) for further information.  
    2. In your terminal, run `git clone https://github.com/Elazrod56/tiktok_json_analyzer.git`  
    3. A new folder called `tiktok_json_analyser` will be created, open it in your terminal.  
    4. Place your TikTok data in `json/` and name it `user_data.json`. Normally it is already named as such  
    5. Finally, in `tiktok_json_analyzer`, run `cargo run -r`. You will need to be connected to the internet to download the cargo crates.  

### Using the released binaries :

	1. : Go into 'releases' and select the latest version  
	2. : Download the executable for your version  
	3. : Place it in an individual folder  
	4. : Make sure that in the same folder there is a `json` folder containing `user_data.json`  
	5. : Open your terminal  
	6. : On Linux, run `./tiktok_json_analyzer_linux-x64`. *(note : you might need to make it executable by typing `chmod +x tiktok_json_analyzer_linux-x64`)*  
	7. : On Windows, run `.\tiktok_json_analyzer_win-x64.exe` (make sure to run it in a terminal window! double-clicking the file will open and close a window too rapidly for you to read the output.)  

## What if you get an error?

If you get a `no such file or directory` error, make sure that the TikTok data (`user_data.json` file) is in the `json` directory.

If you get another error, feel free to open an [Issue](https://github.com/Elazrod56/tiktok_json_analyzer/issues/new).

## Binaries

- The Linux binary was compiled on Arch Linux 64-bit with Rust 1.70.0. It should work on any 64-bit Linux distro.
- The Windows binary was cross-compiled on Arch Linux 64-bit, with Rust 1.70.0. It runs on both Windows 10 and 11.

If the binaries do not run on your machine, consider compiling the code yourself and [opening an Issue](https://github.com/Elazrod56/tiktok_json_analyzer/issues/new).

## For contributors

If you want to contribute to this project, you can contact me on Discord (@elazrod) or open a [PR](https://github.com/Elazrod56/tiktok_json_analyzer/pulls).

## What's the idea behind it?

At the very beginning, I just wanted to see what TikTok data looked like, so I asked a friend to give me his data.
Then I said to myself "Hey, I could code something to calculate stats over this", and I did it.  
  
I first chose Python for simplicity, but since I was learning Rust, I said to myself that
I would do it in Rust to practice. That's why I chose an overkill language for such a simple project.  
  
As of today, I believe that the Analyzer could be a very good way of raising awareness about how TikTok's short-form content messes up your brain and dopaminergic system.
Making people aware of the problem is the first step towards healthier content consumption.

## Future direction

For the moment, I'm rewriting the code to make it less ugly.  

But the project will evolve in the future. I have two big ideas in mind :
- A GUI version : Download -> Run -> Select your file -> Get the results
- A Web App version : Go to the website -> Upload your file -> Get the results

Basically, I intent to make the Analyzer more accessible to the average non-IT person.
