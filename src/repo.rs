use serde::{Deserialize, Serialize};
use comfy_table::Table;

#[derive(Debug, Serialize, Deserialize)]
struct RepoResponse {
    name: String,
    owner: OwnerResponse
}

#[derive(Debug, Serialize, Deserialize)]
struct OwnerResponse {
    url: String
}

pub struct Repo;

impl Repo {
    pub async fn response(user: String) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://api.github.com/users/{set_user}/repos",
            set_user = user
        );

        let response = reqwest::Client::new()
            .get(url)
            .header(reqwest::header::USER_AGENT, "MyInfo")
            .send()
            .await?;

        if response.status().is_success() {
            let result: Vec<RepoResponse> = response.json().await?;
            Self::validation_response(&result)
        } else {
            println!("Request failed with status code: {}", response.status());
        }

        Ok(())
    }

    fn validation_response(responses: &[RepoResponse]) {
        let mut table = Table::new();

        let titles = vec!["Name", "URL"];
        table.set_header(titles);

        for response in responses {
            let rows = vec![
                response.name.to_string(),
                response.owner.url.to_string()
            ];

            table.add_row(rows);
        }

        println!("{table}");
    }
}