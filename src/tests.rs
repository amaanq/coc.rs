#[cfg(test)]
mod tests {
    use bytestream_rs::logiclong::LogicLong;
    use tokio::sync::Mutex;

    use crate::{
        api::{APIError, Client, ConfigForRezponse, Time},
        credentials::CredentialsBuilder,
    };
    use std::{
        env,
        sync::Arc,
        time::{Duration, Instant},
    };

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
    async fn test_login() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;
        // println!("{:#?}", client);
        println!("Time elapsed! {:?}", now.elapsed());

        client.print_keys().await;
    }

    #[tokio::test]
    async fn benchmark_login() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        Client::new(credentials).await;

        println!("Time elapsed! {:?}", now.elapsed());
    }

    #[tokio::test]
    async fn test_player() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await;
        println!("Logged in! {:?}", now.elapsed());
        let tag = "#LQL".to_string();
        let player = client.get_player(tag).await.unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("Player: {:#?}", player);
    }

    #[tokio::test]
    async fn test_clan() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;
        let tag = "#2pp".to_string();

        let clan = client.get_clan(tag).await.expect("Unable to get clan");

        println!("Time elapsed! {:?}", now.elapsed());
        println!("Clan: {:?}", clan);
    }

    #[tokio::test]
    async fn test_current_war() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;
        let tag = "r8j".to_string();

        let war = client.get_current_war(tag).await.unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("War: {:?}", war);
    }

    #[tokio::test]
    async fn test_player_token() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;
        let tag = "#CVJLQOLR".to_string();
        let token = "".to_string();

        let verified = client.get_verified_player(tag, token).await.unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("Verified: {:?}", verified);
    }

    #[tokio::test]
    async fn test_clan_members() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;

        let members = client
            .get_clan_members(
                "#2PP".to_string(),
                ConfigForRezponse {
                    limit: None,
                    time: Some(Time::Before("eyJwb3MiOjF9".to_string())),
                },
            )
            .await
            .unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("{:?}", members.items);
    }

    #[tokio::test]
    async fn test_clan_warlog() {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;

        let war_log = client
            .get_clan_warlog(
                "#R8J".to_string(),
                ConfigForRezponse {
                    limit: None,
                    time: None,
                },
            )
            .await
            .unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("War Log: {:?}", war_log)
    }

    #[test]
    fn test_url() {
        let tag = "#2PP";
        let encoded = urlencoding::encode(tag);
        println!("{}", encoded);
    }

    #[tokio::test]
    async fn test_10000_tags() {
        fn to_tag(low: u32, high: u32) -> String {
            let arr: Vec<char> = vec![
                '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
            ];
            let mut tag = String::new();
            let mut total = low as i64 + high as i64 * 0x100;
            let mut b14;

            while total != 0 {
                b14 = total % 14;
                total /= 14;
                tag.insert(0, arr[b14 as usize]);
            }
            "#".to_owned() + &tag
        }

        let start = 1;
        let end = 2001;
        let vec_ll = (start..end)
            .map(|n| LogicLong {
                low: 0,
                high: n,
                tag: to_tag(0, n),
            })
            .collect::<Vec<_>>();
        println!("done creating logic longs");

        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await;

        //let mut vec_players = Vec::new();
        let mut tasks = Vec::new();
        let throttle_counter = Arc::new(Mutex::new(0));

        let now = Instant::now();
        for logic_long in vec_ll {
            let cloned_client = client.clone();
            let cloned_throttle_counter = throttle_counter.clone();
            let task = tokio::spawn(async move {
                loop {
                    match cloned_client.get_player(logic_long.tag.clone()).await {
                        Ok(_) => break,
                        Err(e) => match e {
                            APIError::BadResponse(_, code) => {
                                if code == reqwest::StatusCode::TOO_MANY_REQUESTS {
                                    *cloned_throttle_counter.lock().await += 1;
                                    //panic!("{}", reason);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        },
                    }
                }
                //println!("{:?}", player);
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap_or_default();
            //std::thread::sleep(Duration::from_micros(1000)); // 2.5ms so 400 requests per second
        }
        println!("Time elapsed! {:?}", now.elapsed());
        println!("Throttle counter: {:#?}", throttle_counter);
    }
}
