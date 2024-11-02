use std::io;
use colored::Colorize;
use indicatif::ProgressBar;
use std::thread::sleep;
use std::time::Duration;

pub struct Header;

impl Header {
    pub fn presentation() {
        let text = r#"
    _____ ______       ___    ___      ___  ________   ________ ________
    |\   _ \  _   \    |\  \  /  /|    |\  \|\   ___  \|\  _____\\   __  \
    \ \  \\\__\ \  \   \ \  \/  / /    \ \  \ \  \\ \  \ \  \__/\ \  \|\  \
     \ \  \\|__| \  \   \ \    / /      \ \  \ \  \\ \  \ \   __\\ \  \\\  \
      \ \  \    \ \  \   \/  /  /        \ \  \ \  \\ \  \ \  \_| \ \  \\\  \
       \ \__\    \ \__\__/  / /           \ \__\ \__\\ \__\ \__\   \ \_______\
        \|__|     \|__|\___/ /             \|__|\|__| \|__|\|__|    \|_______|
                      \|___|/
    "#;

        println!("{}", text.yellow());

        println!("🧰 In this project you will see some information about yourself through the GitHub user");
        println!("🔨 João Lucas");
        println!("💻 https://github.com/joaolfp/MyInfo");
        println!("🔢 0.7.0 Version \n");
    }

    pub fn show_field() -> String {
        println!("Enter the user you want to query:");
        let mut user = String::new();
        io::stdin().read_line(&mut user).expect("Error typing username");

        println!("\n");
        Self::progress_bar();
        println!("\n");

        user
    }

    fn progress_bar() {
        let total_steps = 100;
        let progress_bar = ProgressBar::new(total_steps);

        for _ in 0..total_steps {
            progress_bar.inc(1);
            sleep(Duration::from_millis(50));
        }

        progress_bar.finish_with_message("Done");
    }
}