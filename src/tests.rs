mod tests {
    use tokio::test;

    #[allow(unused_imports)]
    use crate::repo::*;

    #[allow(unused_imports)]
    use crate::info::*;

    #[test]
    async fn test_make_request() {
        let response = reqwest::Client::new()
            .get("https://api.github.com/users/qualquerusuario")
            .header(reqwest::header::USER_AGENT, "MyInfo")
            .send()
            .await;

        assert!(response.is_ok());
    }

    #[test]
    async fn test_response() {
        let result = Info::response("user".to_string()).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_response_repo() {
        let result = Repo::response("user".to_string()).await;
        assert!(result.is_ok());
    }
}