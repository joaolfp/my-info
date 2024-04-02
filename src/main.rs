use reqwest;
use serde::{Deserialize, Serialize};
use std::io;
use comfy_table::Table;
use colored::Colorize;

mod tests;

#[derive(Debug, Serialize, Deserialize)]
struct APIResponse {
    login: String,
    html_url: String,
    name: String,
    followers: i16,
    following: i16
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    presentation();

    let user = header();

    let url = format!(
        "https://api.github.com/users/{set_user}",
        set_user = user
    );

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "MyInfo")
        .send()
        .await?;

    if response.status().is_success() {
        let result: APIResponse = response.json().await?;
        setup_response(&result)
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

fn presentation() {
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
    println!("🔢 0.2.0 Version \n");

}

fn header() -> String {
    println!("Enter the user you want to query:");
    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Error typing username");
    println!("---------------------------------");
    return user
}

fn setup_response(response: &APIResponse) {
    let mut table = Table::new();

    let titles = vec!["Login", "URL", "Name", "Followers", "Following"];

    let rows = vec![
        response.login.to_string(),
        response.html_url.to_string(),
        response.name.to_string(),
        response.followers.to_string(),
        response.following.to_string()];

    table.set_header(titles).add_row(rows);
    println!("{table}");
}