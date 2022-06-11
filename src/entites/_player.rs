use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "townHallLevel")]
    town_hall_level: i64,

    #[serde(rename = "expLevel")]
    exp_level: i64,

    #[serde(rename = "trophies")]
    trophies: i64,

    #[serde(rename = "bestTrophies")]
    best_trophies: i64,

    #[serde(rename = "warStars")]
    war_stars: i64,

    #[serde(rename = "attackWins")]
    attack_wins: i64,

    #[serde(rename = "defenseWins")]
    defense_wins: i64,

    #[serde(rename = "versusTrophies")]
    versus_trophies: i64,

    #[serde(rename = "bestVersusTrophies")]
    best_versus_trophies: i64,

    #[serde(rename = "versusBattleWins")]
    versus_battle_wins: i64,

    #[serde(rename = "role")]
    role: String,

    #[serde(rename = "warPreference")]
    war_preference: String,

    #[serde(rename = "donations")]
    donations: i64,

    #[serde(rename = "donationsReceived")]
    donations_received: i64,

    #[serde(rename = "clan")]
    clan: Option<Clan>,

    #[serde(rename = "achievements")]
    achievements: Vec<Achievement>,

    #[serde(rename = "versusBattleWinCount")]
    versus_battle_win_count: i64,

    #[serde(rename = "labels")]
    labels: Vec<Option<Label>>,

    #[serde(rename = "troops")]
    troops: Vec<Troop>,

    #[serde(rename = "heroes")]
    heroes: Vec<Option<Hero>>,

    #[serde(rename = "spells")]
    spells: Vec<Spell>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "iconUrls")]
    icon_urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hero {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "level")]
    level: i64,

    #[serde(rename = "maxLevel")]
    max_level: i64,

    #[serde(rename = "village")]
    village: Village,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "stars")]
    stars: i64,

    #[serde(rename = "value")]
    value: i64,

    #[serde(rename = "target")]
    target: i64,

    #[serde(rename = "info")]
    info: String,

    #[serde(rename = "completionInfo")]
    completion_info: Option<String>,

    #[serde(rename = "village")]
    village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "clanLevel")]
    clan_level: i64,

    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeUrls {
    #[serde(rename = "small")]
    small: String,

    #[serde(rename = "large")]
    large: String,

    #[serde(rename = "medium")]
    medium: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spell {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "level")]
    level: i64,

    #[serde(rename = "maxLevel")]
    max_level: i64,

    #[serde(rename = "village")]
    village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Troop {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "level")]
    level: i64,

    #[serde(rename = "maxLevel")]
    max_level: i64,

    #[serde(rename = "village")]
    village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Village {
    #[serde(rename = "builderBase")]
    BuilderBase,

    #[serde(rename = "home")]
    Home,
}

impl Player     {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn town_hall_level(&self) -> i64 {
        self.town_hall_level
    }
    pub fn exp_level(&self) -> i64 {
        self.exp_level
    }
    pub fn trophies(&self) -> i64 {
        self.trophies
    }
    pub fn best_trophies(&self) -> i64 {
        self.best_trophies
    }
    pub fn war_stars(&self) -> i64 {
        self.war_stars
    }
    pub fn attack_wins(&self) -> i64 {
        self.attack_wins
    }
    pub fn defense_wins(&self) -> i64 {
        self.defense_wins
    }
    pub fn versus_trophies(&self) -> i64 {
        self.versus_trophies
    }
    pub fn best_versus_trophies(&self) -> i64 {
        self.best_versus_trophies
    }
    pub fn versus_battle_wins(&self) -> i64 {
        self.versus_battle_wins
    }
    pub fn role(&self) -> &str {
        &self.role
    }
    pub fn war_preference(&self) -> &str {
        &self.war_preference
    }
    pub fn donations(&self) -> i64 {
        self.donations
    }
    pub fn donations_received(&self) -> i64 {
        self.donations_received
    }
    pub fn clan(&self) -> &Option<Clan> {
        &self.clan
    }
    pub fn achievements(&self) -> &Vec<Achievement> {
        &self.achievements
    }
    pub fn versus_battle_win_count(&self) -> i64 {
        self.versus_battle_win_count
    }
    pub fn labels(&self) -> &Vec<Option<Label>> {
        &self.labels
    }
    pub fn troops(&self) -> &Vec<Troop> {
        &self.troops
    }
    pub fn heroes(&self) -> &Vec<Option<Hero>> {
        &self.heroes
    }
    pub fn spells(&self) -> &Vec<Spell> {
        &self.spells
    }
}

impl Clan {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn clan_level(&self) -> i64 {
        self.clan_level
    }
    pub fn badge_urls(&self) -> &BadgeUrls {
        &self.badge_urls
    }
}

impl BadgeUrls {
    pub fn small(&self) -> &str {
        &self.small
    }
    pub fn large(&self) -> &str {
        &self.large
    }
    pub fn medium(&self) -> &str {
        &self.medium
    }
}

impl Spell {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn level(&self) -> i64 {
        self.level
    }
    pub fn max_level(&self) -> i64 {
        self.max_level
    }
    pub fn village(&self) -> &Village {
        &self.village
    }
}

impl Achievement {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn stars(&self) -> i64 {
        self.stars
    }
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn target(&self) -> i64 {
        self.target
    }
    pub fn info(&self) -> &str {
        &self.info
    }
    pub fn completion_info(&self) -> &Option<String> {
        &self.completion_info
    }
    pub fn village(&self) -> &Village {
        &self.village
    }
}

impl Troop {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn level(&self) -> i64 {
        self.level
    }
    pub fn max_level(&self) -> i64 {
        self.max_level
    }
    pub fn village(&self) -> &Village {
        &self.village
    }
}
