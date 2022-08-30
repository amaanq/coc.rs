#[cfg(test)]
mod tests {
    use std::{env, sync::Arc, time::Instant};

    use async_trait::async_trait;
    use bytestream_rs::logiclong::LogicLong;
    use time::Month;
    use tokio::sync::Mutex;

    use crate::{
        api::{APIError, Client},
        credentials::CredentialsBuilder,
        models::*,
    };
    use crate::events::EventType;
    use crate::models::player::Player;

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
    async fn test_login() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await?;
        // println!("{:#?}", client);
        println!("Time elapsed! {:?}", now.elapsed());

        client.print_keys().await;
        Ok(())
    }

    #[tokio::test]
    async fn benchmark_login() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        Client::new(credentials).await?;

        println!("Time elapsed! {:?}", now.elapsed());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_warlog() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let clan_warlog = client
            .get_clan_warlog("#2PJP2Q0PY".to_string())
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Clan warlog: {:#?}", clan_warlog);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_clans() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let clans = client
            .get_clans(
                clan_search::ClanSearchOptionsBuilder::new()
                    //.name("hello".to_string())
                    .location_id(location::Local::UnitedStates)
                    .max_members(5)
                    .min_clan_level(20)
                    .build(),
            )
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        clans.items.iter().for_each(|clan| {
            println!("{} - {}", clan.tag, clan.name);
        });
        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_war() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let current_war = client
            .get_current_war("#2L29GJ0G0".to_string())
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Current war: {:#?}", current_war);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let clan = client.get_clan("#90PU0RRG".to_string()).await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Clan: {:?}", clan);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_members() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let clan_members = client.get_clan_members("#2PP".to_string()).await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        // retain clan members where role is Role::CoLeader and print each one when iterating, then collect
        let co_leaders = clan_members
            .items
            .iter()
            .filter(|member| member.role == clan::Role::CoLeader)
            .collect::<Vec<_>>();
        co_leaders.iter().for_each(|member| {
            println!("{} - {}", member.tag, member.name);
        });
        println!("And there are {} co-leaders", co_leaders.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_player() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let mut client = Client::default();
        client.load(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let player = client.get_player("#CGY2G90V".to_string()).await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Player: {:#?}", player);
        Ok(())
    }

    #[tokio::test]
    async fn test_player_token() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();

        let client = Client::new(credentials).await?;
        let token = "".to_string();

        let verified = client
            .verify_player_token("#CVJLQOLR".to_string(), token)
            .await
            .unwrap();

        println!("Time elapsed! {:?}", now.elapsed());
        println!("Verified: {:?}", verified);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_leagues() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let leagues = client.get_leagues().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Leagues: {:#?}", leagues);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_league_season_rankings() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let league_season_rankings = client
            .get_league_season_rankings(
                leagues::LeagueKind::LegendLeague,
                season::SeasonBuilder::new()
                    .year(2015)
                    .month(Month::August)
                    .build(),
                paging::PagingBuilder::new().before(2).build(),
            )
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        league_season_rankings
            .items
            .iter()
            .filter(|ranking| ranking.clan.is_some())
            .for_each(|ranking| {
                let clan = ranking.clan.as_ref().unwrap();
                println!(
                    "We had a clan! {} - {} (Clan: {} - {})",
                    ranking.tag, ranking.name, clan.tag, clan.name
                );
            });

        Ok(())
    }

    #[tokio::test]
    async fn test_get_league() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let league = client
            .get_league(leagues::LeagueKind::LegendLeague)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("League: {:#?}", league);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_war_league() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let war_league = client
            .get_war_league(leagues::WarLeagueKind::ChampionLeagueI)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("War league: {:#?}", war_league);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_war_leagues() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let war_leagues = client.get_war_leagues().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("War leagues: {:#?}", war_leagues);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_rankings() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let mut clan_rankings = client
            .get_clan_rankings(location::Local::UnitedStates)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        clan_rankings
            .items
            .sort_by(|a, b| a.clan_level.cmp(&b.clan_level));
        for c in clan_rankings.items.iter().rev().take(100) {
            println!(
                "{:>3}. {:>9} - {:>15} ({})",
                c.rank, c.tag, c.name, c.clan_level
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_player_rankings() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let player_rankings = client
            .get_player_rankings(location::Local::UnitedStates)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        for p in player_rankings
            .items
            .iter()
            .filter(|player| player.clan.is_some() && player.trophies > 5800)
            .take(100)
        {
            println!(
                "{:>3}. {:>9} - {:>15} ({} - {})",
                p.rank,
                p.tag,
                p.name,
                p.clan.as_ref().unwrap().tag,
                p.clan.as_ref().unwrap().name,
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_versus_clan_rankings() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let mut versus_clan_rankings = client
            .get_versus_clan_rankings(location::Local::UnitedStates)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        versus_clan_rankings
            .items
            .sort_by(|a, b| a.clan_level.cmp(&b.clan_level));
        for c in versus_clan_rankings.items.iter().rev().take(100) {
            println!(
                "{:>3}. {:>9} - {:>15} ({})",
                c.rank, c.tag, c.name, c.clan_level
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_versus_player_rankings() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let mut versus_player_rankings = client
            .get_versus_player_rankings(location::Local::UnitedStates)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        versus_player_rankings
            .items
            .sort_by(|a, b| a.exp_level.cmp(&b.exp_level));
        for c in versus_player_rankings.items.iter().rev().take(100) {
            println!(
                "{:>3}. {:>9} - {:>15} ({})",
                c.rank, c.tag, c.name, c.exp_level
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_locations() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let locations = client.get_locations().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Locations: {:#?}", locations);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_location() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let location = client
            .get_location(location::Local::UnitedStates)
            .await
            .unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Location: {:#?}", location);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_goldpass() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let goldpass = client.get_goldpass().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Goldpass Start: {}", goldpass.start_time());
        println!("Goldpass End: {}", goldpass.end_time());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_player_labels() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let player_labels = client.get_player_labels().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Player Labels: {:#?}", player_labels);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_labels() -> Result<(), APIError> {
        let now = Instant::now();
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = Client::new(credentials).await?;
        println!("Logged in! {:?}", now.elapsed());

        let player_label = client.get_clan_labels().await.unwrap();
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Player Label: {:#?}", player_label);

        Ok(())
    }

    #[test]
    fn test_url() {
        let tag = "#2PP";
        let encoded = urlencoding::encode(tag);
        println!("{}", encoded);
    }

    #[tokio::test]
    async fn test_10000_tags() -> Result<(), APIError> {
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

        let client = Client::new(credentials).await?;

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

        Ok(())
    }


    #[tokio::test]
    async fn test_event() {
        let credentials = CredentialsBuilder::new()
            .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
            .build();
        let client = crate::api::Client::new(credentials).await.unwrap();

        async fn e(client: &Client) {
            let mut x = crate::events::EventsListenerBuilder::new(client);
            x.add_player("#2pp".to_string()).await;

            x.add_clans(vec!["#pp".to_string()]).await
                .build(S)
                .init()
                .await;
        }
        e(&client).await
    }

    struct S;

    #[async_trait]
    impl crate::events::EventHandler for S {
        async fn player(&self, old_player: Option<Player>, new_player: Player) {
            println!("new player")
        }

        async fn handle_error(&self, error: APIError, tag: Option<String>, event_type: EventType) {
            println!("Houston we have a problem!")
        }
    }
}
