use tokio;

#[cfg(test)]
mod tests {
    use crate::api;

    #[tokio::test]
    async fn test_player() {
        let client = api::Client::new(
            // automate making tokens
            std::env::var("COC_TOKEN").expect("Cannot find the specified ENV VAR"),
        );

        let tag = "#2PP".to_string();
        match client.get_player(tag).await {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                api::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                }
                api::ApiError::Api(e) => {
                    if e == reqwest::StatusCode::NOT_FOUND {
                        println!("Not found")
                    } else {
                        println!("Some other variant")
                    }
                }
            },
        }
    }
}
