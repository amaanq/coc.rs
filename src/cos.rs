#[cfg(feature = "cos")]
pub mod cos {
    use reqwest::{StatusCode, Url};

    use crate::cos_models::*;
    use crate::{
        api::{APIError, Client},
        credentials::Credentials,
    };

    impl Client {
        const BASE_COS_URL: &'static str = "https://api.clashofstats.com";
        const COS_LOGIN_ENDPOINT: &'static str = "/login";

        const COS_PLAYERS_ENDPOINT: &'static str = "/players";
        const COS_PLAYERS_HISTORY_ENDPOINT: &'static str = "/history/clans";

        const COS_CLANS_ENDPOINT: &'static str = "/clans";
        const COS_CLANS_PAST_MEMBERS_ENDPOINT: &'static str = "/members/past";

        // Clan War Rankings
        const COS_RANKINGS_CLAN_WAR_WINS_ENDPOINT: &'static str = "/rankings/clans/war-wins";
        const COS_RANKINGS_CLAN_WAR_WIN_STREAK_ENDPOINT: &'static str =
            "/rankings/clans/war-win-streak";
        const COS_RANKINGS_CLAN_BEST_WAR_WIN_STREAK_ENDPOINT: &'static str =
            "/rankings/clans/best-war-win-streak";

        // Clan Trophy Rankings
        const COS_RANKINGS_CLAN_TROPHIES_ENDPOINT: &'static str = "/rankings/clans/trophies";
        const COS_RANKINGS_CLAN_VERSUS_TROPHIES_ENDPOINT: &'static str = "/rankings/clans/versus";

        // Player Trophy Rankings
        const COS_RANKINGS_PLAYER_TROPHIES_ENDPOINT: &'static str = "/rankings/players/trophies";
        const COS_RANKINGS_PLAYER_VERSUS_TROPHIES_ENDPOINT: &'static str =
            "/rankings/players/versus";
        const COS_RANKINGS_PLAYER_BEST_TROPHIES_ENDPOINT: &'static str =
            "/rankings/players/best-trophies";
        const COS_RANKINGS_PLAYER_BEST_VERSUS_TROPHIES_ENDPOINT: &'static str =
            "/rankings/players/best-versus";
        const COS_RANKINGS_PLAYER_LEGEND_TROPHIES: &'static str =
            "/rankings/players/legend/trophies";

        // Player War Star Rankings
        const COS_RANKINGS_PLAYER_WAR_STARS: &'static str = "/rankings/players/war-stars";
        const COS_RANKINGS_PLAYER_CWL_WAR_STARS: &'static str =
            "/rankings/players/war-league-legend";

        // Player Multiplayer Rankings
        const COS_RANKINGS_PLAYER_ATTACK_WINS: &'static str = "/rankings/players/attack-wins";
        const COS_RANKINGS_PLAYER_DEFENSE_WINS: &'static str = "/rankings/players/defense-wins";
        const COS_RANKINGS_PLAYER_VERSUS_BATTLE_WINS: &'static str =
            "/rankings/players/versus-battle-wins";
        const COS_RANKINGS_PLAYER_HEROIC_HEIST: &'static str = "/rankings/players/heroic-heist";
        const COS_RANKINGS_PLAYER_CONQUEROR: &'static str = "/rankings/players/conqueror";
        const COS_RANKINGS_PLAYER_UNBREAKABLE: &'static str = "/rankings/players/unbreakable";
        const COS_RANKINGS_PLAYER_HUMILIATOR: &'static str = "/rankings/players/humiliator";
        const COS_RANKINGS_PLAYER_UN_BUILD_IT: &'static str = "/rankings/players/un-build-it";

        // Player Social Rankings
        const COS_RANKINGS_PLAYER_GAMES_CHAMPION: &'static str = "/rankings/players/games-champion";
        const COS_RANKINGS_PLAYER_TROOPS_DONATED: &'static str = "/rankings/players/donations";
        const COS_RANKINGS_PLAYER_TROOPS_RECEIVED: &'static str =
            "/rankings/players/donations-received";
        const COS_RANKINGS_PLAYER_FRIEND_IN_NEED: &'static str = "/rankings/players/friends-in-need";

        // Player Single Player Rankings
        const COS_RANKINGS_PLAYER_EXP_LEVEL: &'static str = "/rankings/players/exp-level";
        const COS_RANKINGS_PLAYER_WELL_SEASONED: &'static str = "/rankings/players/well-seasoned";
        const COS_RANKINGS_PLAYER_GET_THOSE_GOBLINS: &'static str =
            "/rankings/players/get-those-goblins";
        const COS_RANKINGS_PLAYER_NICE_AND_TIDY: &'static str = "/rankings/players/nice-and-tidy";

        /// While credentials **are** passed in, only the first one will be used so ensure it's a freshly built instance
        pub async fn cos_login(&self, credentials: &Credentials) -> Result<(), APIError> {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Serialize, Deserialize)]
            struct LoginResponse {
                success: bool,
            }

            let url = format!("{}{}", Self::BASE_COS_URL, Self::COS_LOGIN_ENDPOINT);
            let body =
                serde_json::to_string(credentials.0.get(0).expect("Your credentials are empty"))
                    .unwrap();

            let _login: LoginResponse = self.parse_json(self.cos_post(url, body)).await?;

            Ok(())
        }

        pub async fn cos_get_player(
            &self,
            player_tag: String,
        ) -> Result<cos_player::Player, APIError> {
            let url = format!(
                "{}{}/{}",
                Self::BASE_COS_URL,
                Self::COS_PLAYERS_ENDPOINT,
                urlencoding::encode(player_tag.as_str())
            );
            println!("url: {}", url);
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_history(
            &self,
            player_tag: String,
        ) -> Result<cos_player_history::PlayerHistory, APIError> {
            let url = format!(
                "{}{}/{}{}",
                Self::BASE_COS_URL,
                Self::COS_PLAYERS_ENDPOINT,
                urlencoding::encode(player_tag.as_str()),
                Self::COS_PLAYERS_HISTORY_ENDPOINT
            );
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_clan(&self, clan_tag: String) -> Result<cos_clan::Clan, APIError> {
            let url = format!(
                "{}{}/{}",
                Self::BASE_COS_URL,
                Self::COS_CLANS_ENDPOINT,
                urlencoding::encode(clan_tag.as_str())
            );
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_clan_past_members(
            &self,
            clan_tag: String,
        ) -> Result<cos_clan_history::ClanPastMembers, APIError> {
            let url = format!(
                "{}{}/{}{}",
                Self::BASE_COS_URL,
                Self::COS_CLANS_ENDPOINT,
                urlencoding::encode(clan_tag.as_str()),
                Self::COS_CLANS_PAST_MEMBERS_ENDPOINT
            );
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_war_wins_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::ClanLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_CLAN_WAR_WINS_ENDPOINT,
                ),
                options.build_for_clan(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_war_win_streak_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::ClanLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_CLAN_WAR_WIN_STREAK_ENDPOINT,
                ),
                options.build_for_clan(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_best_war_win_streak_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::ClanLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_CLAN_BEST_WAR_WIN_STREAK_ENDPOINT,
                ),
                options.build_for_clan(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_clan_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::ClanLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_CLAN_TROPHIES_ENDPOINT,
                ),
                options.build_for_clan(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_clan_versus_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::ClanLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_CLAN_VERSUS_TROPHIES_ENDPOINT,
                ),
                options.build_for_clan(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_TROPHIES_ENDPOINT,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_versus_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_VERSUS_TROPHIES_ENDPOINT,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_best_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_BEST_TROPHIES_ENDPOINT,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_best_versus_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_BEST_VERSUS_TROPHIES_ENDPOINT,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_legend_trophies_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::LegendsLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_LEGEND_TROPHIES,
                ),
                options.build_for_legends(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_war_stars_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_WAR_STARS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_cwl_war_stars_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_CWL_WAR_STARS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_attack_wins_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_ATTACK_WINS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_defense_wins_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_DEFENSE_WINS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_versus_battle_wins_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_VERSUS_BATTLE_WINS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_heroic_heist_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_HEROIC_HEIST,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_conqueror_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_CONQUEROR,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_unbreakable_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_UNBREAKABLE,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_humiliator_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_HUMILIATOR,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_un_build_it_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_UN_BUILD_IT,
                ),
                options.build_for_builder(),
            )
            .unwrap();
            println!("{}", url);
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_games_champion_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_GAMES_CHAMPION,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_troops_donated_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_TROOPS_DONATED,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_troops_received_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_TROOPS_RECEIVED,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_friend_in_need_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_FRIEND_IN_NEED,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_exp_level_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_EXP_LEVEL,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_well_seasoned_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_WELL_SEASONED,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_get_those_goblins_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_GET_THOSE_GOBLINS,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }

        pub async fn cos_get_player_nice_and_tidy_leaderboard(
            &self,
            options: cos_options::Options,
        ) -> Result<leaderboard::PlayerLeaderboard, APIError> {
            let url = Url::parse_with_params(
                &format!(
                    "{}{}",
                    Self::BASE_COS_URL,
                    Self::COS_RANKINGS_PLAYER_NICE_AND_TIDY,
                ),
                options.build_for_player(),
            )
            .unwrap();
            self.parse_json(self.cos_get(url)).await
        }
    }
}
