use tokio;
#[cfg(test)]
mod tests {
    use crate::http;

    #[tokio::test]
    async fn test_player(){
        let client = http::Clinet::new(std::env::var("COC_TOKEN").expect("Cannot find the specified ENV VAR"));

        let tag = "#2PP".to_string();
        match client.get_player(tag).await {
            Ok(body) => {
                println!("{:?}", body);
                assert!(true);
            }
            Err(e) => {
                match e {
                    http::ApiError::Request(e) => {
                        println!("Request - {:?}", e);
                        assert!(false);
                    }
                    http::ApiError::Api(e) => {
                        println!("Api Error - {:?}", e);
                        assert!(false);
                    }
                }
            }
        }
    }
}