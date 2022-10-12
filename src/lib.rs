/// Base API wrapper
pub mod api;
mod models;
pub use models::*;

/// Clash of Stats API wrapper
mod clash_of_stats;
pub use clash_of_stats::*;
mod cos_models;
pub use cos_models::*;

pub mod credentials;
/// Developer Site API wrapper
mod dev;

/// Events to track changes
pub mod events;

/// To access crates that are interconnected with coc.rs without adding it as a dependency
pub mod prelude;

#[macro_use]
extern crate num_derive;

#[cfg(test)]
mod tests {
    use std::{env, sync::Arc, time::Instant};

    use anyhow::{Context, Result};
    use async_trait::async_trait;
    use logic_long::LogicLong;
    use time::Month;
    use tokio::sync::Mutex;

    use crate::{
        api::{APIError, Client},
        credentials::Credentials,
        events::{EventHandler, EventType, EventsListenerBuilder},
        location::Local,
        models::*,
    };

    static mut LOADED: bool = false;

    lazy_static::lazy_static!(
        pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::default());
    );

    /// Get an environment variable, returning an Err with a
    /// nice error message mentioning the missing variable in case the value is not found.
    fn required_env_var(key: &str) -> Result<String> {
        env::var(key).with_context(|| format!("Missing environment variable {}", key))
    }

    async fn load_client() -> anyhow::Result<()> {
        unsafe {
            if !LOADED {
                let credentials = Credentials::builder();
                let credentials = required_env_var("emails")?
                    .split(',')
                    .map(std::string::ToString::to_string)
                    .zip(
                        required_env_var("passwords")?
                            .split(',')
                            .map(std::string::ToString::to_string),
                    )
                    .fold(credentials, |credentials, (email, password)| {
                        credentials.add_credential(email, password)
                    })
                    .build();

                CLIENT.lock().await.load(credentials).await?;
                LOADED = true;
            }
        }
        Ok(())
    }

    #[test]
    fn test_credentials() {
        let credentials = Credentials::builder()
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
    async fn test_get_clan_warlog() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let clan_warlog = CLIENT.lock().await.get_clan_warlog("#2PJP2Q0PY").await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Clan warlog: {:#?}", clan_warlog);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_clans() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let clans = CLIENT
            .lock()
            .await
            .get_clans(
                clan_search::ClanSearchOptionsBuilder::new()
                    .location_id(location::Local::UnitedStates)
                    .max_members(5)
                    .min_clan_level(20)
                    .build(),
            )
            .await?;
        println!("Time elapsed! {:?}", now.elapsed());

        clans.items.iter().for_each(|clan| {
            println!("{} - {}", clan.tag, clan.name);
        });
        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_war() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let current_war = CLIENT.lock().await.get_current_war("#2L29GJ0G0").await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Current war: {:#?}", current_war);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let clan = CLIENT.lock().await.get_clan("#90PU0RRG").await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Clan: {:?}", clan);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_members() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let clan_members = CLIENT.lock().await.get_clan_members("#2PP").await?;
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
    async fn test_get_clan_capital_raid_seasons() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let clan_capital_raid_seasons =
            CLIENT.lock().await.get_clan_capital_raid_seasons("#2PP").await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Clan capital raid seasons: {:#?}", clan_capital_raid_seasons);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_player() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let player = CLIENT.lock().await.get_player("#2PP").await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Player: {:#?}", player);
        #[cfg(feature = "extra")]
        println!("Hero Pets: {:#?}", player.hero_pets());
        Ok(())
    }

    #[tokio::test]
    async fn test_player_token() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let verified = CLIENT.lock().await.verify_player_token("#CVJLQOLR", "").await?;

        println!("Time elapsed! {:?}", now.elapsed());
        println!("Verified: {:?}", verified);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_leagues() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let leagues = CLIENT.lock().await.get_leagues().await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Leagues: {:#?}", leagues);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_league_season_rankings() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        println!("Logged in! {:?}", now.elapsed());

        let league_season_rankings = CLIENT
            .lock()
            .await
            .get_league_season_rankings(
                leagues::LeagueKind::LegendLeague,
                season::Season::builder().year(2015).month(Month::August).build(),
                paging::Paging::builder().before(2).build(),
            )
            .await?;
        println!("Time elapsed! {:?}", now.elapsed());

        league_season_rankings.items.iter().for_each(|ranking| {
            if let Some(clan) = ranking.clan.as_ref() {
                println!(
                    "We had a clan! {} - {} (Clan: {} - {})",
                    ranking.tag, ranking.name, clan.tag, clan.name
                );
            }
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_get_league() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let league = CLIENT.lock().await.get_league(leagues::LeagueKind::LegendLeague).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("League: {:#?}", league);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_league_seasons() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let league_seasons =
            CLIENT.lock().await.get_league_seasons(leagues::LeagueKind::LegendLeague).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        league_seasons.items.iter().for_each(|season| {
            println!("Season: {}", season);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_get_war_league() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let war_league =
            CLIENT.lock().await.get_war_league(leagues::WarLeagueKind::ChampionLeagueI).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("War league: {:#?}", war_league);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_war_leagues() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let war_leagues = CLIENT.lock().await.get_war_leagues().await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("War leagues: {:#?}", war_leagues);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_rankings() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let mut clan_rankings =
            CLIENT.lock().await.get_clan_rankings(location::Local::UnitedStates).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        clan_rankings.items.sort_by(|a, b| a.clan_level.cmp(&b.clan_level));
        for c in clan_rankings.items.iter().rev().take(100) {
            println!("{:>3}. {:>9} - {:>15} ({})", c.rank, c.tag, c.name, c.clan_level);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_player_rankings() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let player_rankings =
            CLIENT.lock().await.get_player_rankings(location::Local::UnitedStates).await?;
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
    async fn test_get_versus_clan_rankings() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let mut versus_clan_rankings =
            CLIENT.lock().await.get_versus_clan_rankings(location::Local::UnitedStates).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        versus_clan_rankings.items.sort_by(|a, b| a.clan_level.cmp(&b.clan_level));
        for c in versus_clan_rankings.items.iter().rev().take(100) {
            println!("{:>3}. {:>9} - {:>15} ({})", c.rank, c.tag, c.name, c.clan_level);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_versus_player_rankings() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let mut versus_player_rankings =
            CLIENT.lock().await.get_versus_player_rankings(location::Local::UnitedStates).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        versus_player_rankings.items.sort_by(|a, b| a.exp_level.cmp(&b.exp_level));
        for c in versus_player_rankings.items.iter().rev().take(100) {
            println!("{:>3}. {:>9} - {:>15} ({})", c.rank, c.tag, c.name, c.exp_level);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_locations() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let locations = CLIENT.lock().await.get_locations().await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Locations: {:#?}", locations);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_location() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let location = CLIENT.lock().await.get_location(location::Local::UnitedStates).await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Location: {:#?}", location);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_goldpass() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let goldpass = CLIENT.lock().await.get_goldpass().await?;
        println!("Time elapsed! {:?}", now.elapsed());

        println!("Goldpass Start: {}", goldpass.start_time());
        println!("Goldpass End: {}", goldpass.end_time());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_player_labels() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let player_labels = CLIENT.lock().await.get_player_labels().await?;
        println!("Time elapsed! {:?}", now.elapsed());
        println!("Player Labels: {:#?}", player_labels);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clan_labels() -> anyhow::Result<()> {
        let now = Instant::now();

        load_client().await?;

        let player_label = CLIENT.lock().await.get_clan_labels().await?;
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
    async fn test_10000_tags() -> anyhow::Result<()> {
        fn to_tag(low: u32, high: u32) -> String {
            let arr: Vec<char> =
                vec!['0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V'];
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
        let end = 101;
        let vec_ll = (start..end)
            .map(|n| LogicLong {
                // putting this here as a reminder to add a pure LogicLong::random() method
                low: 0,
                high: n,
                tag: to_tag(0, n),
            })
            .collect::<Vec<_>>();
        println!("done creating logic longs");

        load_client().await?;

        // hold the lock for the entire test so it's not dropped and another test picks it up
        let client = CLIENT.lock().await;

        let mut tasks = Vec::new();
        let throttle_counter = Arc::new(Mutex::new(0));

        let now = Instant::now();
        for logic_long in vec_ll {
            let client = client.clone();
            let cloned_throttle_counter = throttle_counter.clone();
            let task = tokio::spawn(async move {
                loop {
                    match client.get_player(&logic_long.tag).await {
                        Ok(_) => break,
                        Err(e) => match e {
                            APIError::BadResponse(_, code) => {
                                if code == reqwest::StatusCode::TOO_MANY_REQUESTS {
                                    *cloned_throttle_counter.lock().await += 1;
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        },
                    }
                }
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await?;
        }
        println!("Time elapsed! {:?}", now.elapsed());
        println!("Throttle counter: {:#?}", throttle_counter);

        Ok(())
    }

    #[tokio::test]
    async fn test_event() -> anyhow::Result<()> {
        load_client().await?;

        struct S;

        #[async_trait]
        impl EventHandler for S {
            async fn player(&self, old_player: Option<player::Player>, new_player: player::Player) {
                println!(
                    "From {} to {}",
                    {
                        if let Some(old_player) = old_player {
                            old_player.tag
                        } else {
                            String::new()
                        }
                    },
                    new_player.tag
                );
            }

            async fn handle_error(&self, error: APIError, tag: String, event_type: EventType) {
                println!("Houston we have a problem! {} with {} @ {}", error, tag, event_type);
            }
        }

        let task = tokio::spawn(async move {
            let events_listener = EventsListenerBuilder::new(CLIENT.lock().await.clone())
                .add_player("#2PP")
                .add_clans(vec!["#2PP"])
                .build(S);
            events_listener.start(Some(std::time::Duration::from_secs(5))).await
        });

        task.await??;

        Ok(())
    }

    #[test]
    fn test_primitive_to_local() {
        let local = Local::from_i32(32000249);
        assert_eq!(local, Some(Local::UnitedStates));
    }

    #[cfg(feature = "cos")]
    #[cfg(test)]
    mod tests_cos {
        use std::{env, time::Instant};

        use crate::{api::APIError, cos_options, credentials::Credentials, location::Local};

        #[tokio::test]
        async fn test_cos_login() -> Result<(), APIError> {
            let now = Instant::now();
            let credentials = Credentials::builder()
                .add_credential(env::var("cosemail").unwrap(), env::var("cospassword").unwrap())
                .build();
            super::CLIENT.lock().await.cos_login(&credentials).await.unwrap();

            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player = super::CLIENT.lock().await.cos_get_player("#2PP").await.unwrap();

            println!("{:?}", cos_player);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_history() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_history =
                super::CLIENT.lock().await.cos_get_player_history("#2PP").await.unwrap();
            println!("{:?}", cos_player_history);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_clan() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan = super::CLIENT.lock().await.cos_get_clan("#2PP").await.unwrap();
            println!("{:?}", cos_clan);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_clan_past_members() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_history =
                super::CLIENT.lock().await.cos_get_clan_past_members("#2PP").await.unwrap();
            println!("{:?}", cos_clan_history);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_war_wins_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_history = super::CLIENT
                .lock()
                .await
                .cos_get_war_wins_leaderboard(
                    cos_options::Options::builder().location(Local::None).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_clan_history);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_war_win_streak_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_history = super::CLIENT
                .lock()
                .await
                .cos_get_war_win_streak_leaderboard(
                    cos_options::Options::builder().location(Local::None).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_clan_history);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_best_war_win_streak_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_history = super::CLIENT
                .lock()
                .await
                .cos_get_best_war_win_streak_leaderboard(
                    cos_options::Options::builder().location(Local::None).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_clan_history);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_clan_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_ranking = super::CLIENT
                .lock()
                .await
                .cos_get_clan_trophies_leaderboard(
                    cos_options::Options::builder().location(Local::None).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_clan_ranking);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_clan_versus_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_clan_ranking = super::CLIENT
                .lock()
                .await
                .cos_get_clan_versus_trophies_leaderboard(
                    cos_options::Options::builder().location(Local::None).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_clan_ranking);
            println!("Time elapsed! {:?}", now.elapsed());

            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_ranking = super::CLIENT
                .lock()
                .await
                .cos_get_player_trophies_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_ranking);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_versus_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_ranking = super::CLIENT
                .lock()
                .await
                .cos_get_player_versus_trophies_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_ranking);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_best_versus_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_ranking = super::CLIENT
                .lock()
                .await
                .cos_get_player_best_versus_trophies_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_ranking);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_legend_trophies_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_legend_trophies_leaderboard(
                    cos_options::Options::builder().page(5555).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_war_stars_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_war_stars_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_cwl_war_stars_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_cwl_war_stars_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_attack_wins_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_attack_wins_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_defense_wins_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_defense_wins_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        //cos_get_player_versus_battle_wins_leaderboard
        #[tokio::test]
        async fn test_cos_get_player_versus_battle_wins_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_versus_battle_wins_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_heroic_heist_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_heroic_heist_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_conqueror_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_conqueror_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_unbreakable_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_unbreakable_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_humiliator_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_humiliator_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_un_build_it_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_un_build_it_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_games_champion_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_games_champion_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_troops_donated_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_troops_donated_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_troops_received_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_troops_received_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_friend_in_need_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_friend_in_need_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_exp_level_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_exp_level_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_well_seasoned_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_well_seasoned_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_get_those_goblins_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_get_those_goblins_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }

        #[tokio::test]
        async fn test_cos_get_player_nice_and_tidy_leaderboard() -> Result<(), APIError> {
            let now = Instant::now();

            let cos_player_versus_trophies_history = super::CLIENT
                .lock()
                .await
                .cos_get_player_nice_and_tidy_leaderboard(
                    cos_options::Options::builder().location(Local::UnitedStates).page(1).build(),
                )
                .await
                .unwrap();
            println!("{:?}", cos_player_versus_trophies_history);
            println!("Time elapsed! {:?}", now.elapsed());
            Ok(())
        }
    }
}
