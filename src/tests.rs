mod tests {
    use tokio::test;
    
    #[allow(unused_imports)]
    use crate::response;

    #[allow(unused_imports)]
    use crate::response_repo;

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
        let result = response("user".to_string()).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_response_repo() {
       let result = response_repo("user".to_string()).await;
       assert!(result.is_ok());
   }
}