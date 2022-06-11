use tokio;

#[cfg(test)]
mod tests {
    use crate::http;

    #[tokio::test]
    async fn test_player() {
        let client = http::Client::new(
            std::env::var("COC_TOKEN").expect("Cannot find the specified ENV VAR"),
        );

        let tag = "#2PP".to_string();
        match client.get_player(tag).await {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                http::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                }
                http::ApiError::Api(e) => {
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
