#[cfg(test)]
mod tests {
    use std::mem::transmute;
    use crate::http;

    #[test]
    fn test_player() {
        let client = http::Client::new(
            std::env::var("COC_TOKEN").unwrap(),
        );

        let tag = "#2PP".to_string();
        match client.get_player(tag) {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                http::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                    assert!(true);
                }
                http::ApiError::Api(e) => {
                    if e == reqwest::StatusCode::NOT_FOUND {
                        println!("Not found")
                    } else {
                        println!("Some other variant {}", e.as_u16());
                    }
                    assert!(false);
                }
            },
        }
    }

    #[test]
    fn test_clan(){
        let client = http::Client::new(
            std::env::var("COC_TOKEN").unwrap(),
        );
        let tag = "#2pp".to_string();

        let x = client.get_clan(tag).expect("Unable to get clan");

        println!("{:?}", x);
        assert!(true)
    }

    #[test]
    fn test_current_war(){
        let client = http::Client::new(
            std::env::var("COC_TOKEN").unwrap(),
        );
        let tag = "r8j".to_string();

        let x = client.get_current_war(tag);
        match x {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                http::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                    assert!(true)
                }
                http::ApiError::Api(e) => {
                    if e == reqwest::StatusCode::NOT_FOUND {
                        println!("Not found")
                    }else {
                        println!("Some other variant {}", e.as_u16())
                    }
                    assert!(false)
                }
            },
        }
    }

    #[test]
    fn test_player_token(){
        let client = http::Client::new(
            std::env::var("COC_TOKEN").unwrap(),
        );
        let tag = "#CVJLQOLR".to_string();
        let token = "".to_string();

        let x = client.get_verified_player(tag, token);
        match x {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                http::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                }
                http::ApiError::Api(e) => {
                    if e == reqwest::StatusCode::NOT_FOUND {
                        println!("Not found");
                        assert!(true)
                    }else {
                        println!("Some other variant {}", e.as_u16())
                    }
                    assert!(false)
                }
            },
        }
    }

}
