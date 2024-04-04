use reqwest;
use serde::{Deserialize, Serialize};
use comfy_table::Table;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    login: String,
    html_url: String,
    name: String,
    followers: i16,
    following: i16
}

pub async fn response(user: String) -> Result<(), reqwest::Error> {
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