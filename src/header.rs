use colored::Colorize;
use demand::Input;

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
        println!("🔢 0.8.0 Version \n");
    }

    pub fn show_field() -> Result<String, String> {
        let validation_empty = |s: &str| {
            if s.trim().is_empty() {
                Err("Username cannot be empty")
            } else {
                Ok(())
            }
        };

        let input = Input::new("Username")
            .placeholder("Enter the user you want to query:")
            .prompt("Username: ")
            .validation(validation_empty);

        match input.run() {
            Ok(username) => Ok(username),
            Err(err) => {
                if err.to_string().contains("interrupted") {
                    println!("{}", "Bye Bye!!".blue());
                    Err("".to_string())
                } else {
                    println!("{}", format!("Error: {}", err).red());
                    Err("".to_string())
                }
            }
        }
    }
}