#[cfg(test)]
mod tests {
    use tokio::test;

    #[test]
    async fn test_make_request() {
        let response = reqwest::Client::new()
            .get("https://api.github.com/users/qualquerusuario")
            .header(reqwest::header::USER_AGENT, "MyInfo")
            .send()
            .await;

        assert!(response.is_ok());
    }
}