#[cfg(feature = "cos")]
#[cfg(test)]
mod tests {
    use std::{env, time::Instant};

    use crate::{
        api::{APIError, Client},
        cos_options,
        credentials::Credentials,
        location::Local,
    };

    #[tokio::test]
    async fn test_cos_login() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();
        let credentials = Credentials::builder()
            .add_credential(
                env::var("cosemail").unwrap(),
                env::var("cospassword").unwrap(),
            )
            .build();
        client.cos_login(&credentials).await.unwrap();

        println!("Time elapsed! {:?}", now.elapsed());

        Ok(())
    }

    #[tokio::test]
    async fn test_cos_get_player() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();

        let cos_player = client.cos_get_player("#2PP".to_string()).await.unwrap();

        println!("{:?}", cos_player);
        println!("Time elapsed! {:?}", now.elapsed());

        Ok(())
    }

    #[tokio::test]
    async fn test_cos_get_player_history() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();

        let cos_player_history = client
            .cos_get_player_history("#2PP".to_string())
            .await
            .unwrap();
        println!("{:?}", cos_player_history);
        println!("Time elapsed! {:?}", now.elapsed());

        Ok(())
    }

    #[tokio::test]
    async fn test_cos_get_clan() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();

        let cos_clan = client.cos_get_clan("#2PP".to_string()).await.unwrap();
        println!("{:?}", cos_clan);
        println!("Time elapsed! {:?}", now.elapsed());

        Ok(())
    }

    #[tokio::test]
    async fn test_cos_get_clan_past_members() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();

        let cos_clan_history = client
            .cos_get_clan_past_members("#2PP".to_string())
            .await
            .unwrap();
        println!("{:?}", cos_clan_history);
        println!("Time elapsed! {:?}", now.elapsed());

        Ok(())
    }

    #[tokio::test]
    async fn test_cos_get_war_wins_leaderboard() -> Result<(), APIError> {
        let now = Instant::now();
        let client = Client::default();

        let cos_clan_history = client
            .cos_get_war_wins_leaderboard(
                cos_options::Options::builder()
                    .location(Local::None)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_clan_history = client
            .cos_get_war_win_streak_leaderboard(
                cos_options::Options::builder()
                    .location(Local::None)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_clan_history = client
            .cos_get_best_war_win_streak_leaderboard(
                cos_options::Options::builder()
                    .location(Local::None)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_clan_ranking = client
            .cos_get_clan_trophies_leaderboard(
                cos_options::Options::builder()
                    .location(Local::None)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_clan_ranking = client
            .cos_get_clan_versus_trophies_leaderboard(
                cos_options::Options::builder()
                    .location(Local::None)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_ranking = client
            .cos_get_player_trophies_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_ranking = client
            .cos_get_player_versus_trophies_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_ranking = client
            .cos_get_player_best_versus_trophies_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_legend_trophies_leaderboard(
                cos_options::Options::builder().page(1).build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_war_stars_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_cwl_war_stars_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_attack_wins_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_defense_wins_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_versus_battle_wins_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_heroic_heist_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_conqueror_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_unbreakable_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_humiliator_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_un_build_it_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_games_champion_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_troops_donated_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_troops_received_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_friend_in_need_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_exp_level_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_well_seasoned_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_get_those_goblins_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
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
        let client = Client::default();

        let cos_player_versus_trophies_history = client
            .cos_get_player_nice_and_tidy_leaderboard(
                cos_options::Options::builder()
                    .location(Local::UnitedStates)
                    .page(1)
                    .build(),
            )
            .await
            .unwrap();
        println!("{:?}", cos_player_versus_trophies_history);
        println!("Time elapsed! {:?}", now.elapsed());
        Ok(())
    }
}
