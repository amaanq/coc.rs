#[cfg(test)]
mod tests {
    use crate::{api::Client, credentials::CredentialsBuilder};
    use std::{env, time::Instant};

    #[test]
    fn test_credentials() {
        let credentials = CredentialsBuilder::new()
            .add_credential("user1".to_owned(), "pass1".to_owned())
            .add_credential("user2".to_owned(), "pass2".to_owned())
            .build();
        assert_eq!(credentials.0.len(), 2);
        assert_eq!(credentials.0[0].email(), "user1");
        assert_eq!(credentials.0[0].password(), "pass1");
        assert_eq!(credentials.0[1].email(), "user2");
        assert_eq!(credentials.0[1].password(), "pass2");
    }

    #[tokio::test]
    async fn benchmark_login() {
        println!("starting");
        let now = Instant::now();
        // load .env file
        //Client::new(env::var("username").unwrap(), env::var("password").unwrap()).await;

        println!("Elapsed: {:.2?}", now.elapsed());
    }

    // #[tokio::test]
    // async fn test_player() {
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //
    //     let tag = "#2PP".to_string();
    //     match client.get_player(tag).await {
    //         Ok(body) => {
    //             println!("{:?}", body);
    //         }
    //         Err(e) => match e {
    //             api::ApiError::Request(e) => {
    //                 println!("Request - {:?}", e);
    //                 assert!(true);
    //             }
    //             api::ApiError::Api(e) => {
    //                 if e == reqwest::StatusCode::NOT_FOUND {
    //                     println!("Not found")
    //                 } else {
    //                     println!("Some other variant {}", e.as_u16());
    //                 }
    //                 assert!(false);
    //             }
    //         },
    //     }
    // }
    //
    // #[tokio::test]
    // async fn test_clan(){
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //     let tag = "#2pp".to_string();
    //
    //     let x = client.get_clan(tag).await.expect("Unable to get clan");
    //
    //     println!("{:?}", x);
    //     assert!(true)
    // }
    //
    // #[tokio::test]
    // async fn test_current_war(){
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //     let tag = "r8j".to_string();
    //
    //     let x = client.get_current_war(tag);
    //     match x.await {
    //         Ok(body) => {
    //             println!("{:?}", body);
    //         }
    //         Err(e) => match e {
    //             api::ApiError::Request(e) => {
    //                 println!("Request - {:?}", e);
    //                 assert!(true)
    //             }
    //             api::ApiError::Api(e) => {
    //                 if e == reqwest::StatusCode::NOT_FOUND {
    //                     println!("Not found")
    //                 }else {
    //                     println!("Some other variant {}", e.as_u16())
    //                 }
    //                 assert!(false)
    //             }
    //         },
    //     }
    // }
    //
    // #[tokio::test]
    // async fn test_player_token(){
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //     let tag = "#CVJLQOLR".to_string();
    //     let token = "".to_string();
    //
    //     let x = client.get_verified_player(tag, token);
    //     match x.await {
    //         Ok(body) => {
    //             println!("{:?}", body);
    //         }
    //         Err(e) => match e {
    //             api::ApiError::Request(e) => {
    //                 println!("Request - {:?}", e);
    //             }
    //             api::ApiError::Api(e) => {
    //                 if e == reqwest::StatusCode::NOT_FOUND {
    //                     println!("Not found");
    //                     assert!(true)
    //                 }else {
    //                     println!("Some other variant {}", e.as_u16())
    //                 }
    //                 assert!(false)
    //             }
    //         },
    //     }
    // }
    //
    // #[tokio::test]
    // async fn test_clan_members(){
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //
    //     let x = client.get_clan_members(
    //         "2pp".to_string(),
    //         ConfigForRezponse {
    //             limit: None,
    //             time: Some(api::Time::Before("eyJwb3MiOjF9".to_string()))
    //         }
    //     ).await.unwrap();
    //
    //     println!("{:?}", x)
    //
    // }
    //
    // #[tokio::test]
    // async fn test_clan_warlog(){
    //     let client = api::Client::new(
    //         std::env::var("COC_TOKEN").unwrap(),
    //     );
    //
    //     let x = client.get_clan_warlog(
    //         "r8j".to_string(),
    //         ConfigForRezponse {
    //             limit: None,
    //             time: None
    //         }
    //     ).await.unwrap();
    //
    //     println!("{:?}", x)
    //
    // }
}
