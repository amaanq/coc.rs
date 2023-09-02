use serde::{Deserialize, Serialize};

use super::{badge_urls, clan, labels, leagues, season};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub tag: String,
    pub name: String,
    pub town_hall_level: i8,
    pub town_hall_weapon_level: Option<i8>,
    pub exp_level: i32,
    pub trophies: i32,
    pub best_trophies: i32,
    pub war_stars: i32,
    pub attack_wins: i32,
    pub defense_wins: i32,
    pub builder_hall_level: Option<i8>,
    pub versus_trophies: i32,
    pub best_versus_trophies: i32,
    pub versus_battle_wins: i32,
    pub role: Option<clan::Role>,
    pub war_preference: Option<WarPreference>,
    pub donations: i32,
    pub donations_received: i32,
    pub clan_capital_contributions: i32,
    pub clan: Option<PlayerClan>,
    pub league: Option<leagues::League>,
    pub legend_statistics: Option<LegendStatistics>,
    pub achievements: Vec<Achievement>,
    pub versus_battle_win_count: Option<i32>,
    pub labels: Vec<labels::PlayerLabel>,
    pub troops: Vec<Troop>,
    pub heroes: Vec<Hero>,
    pub spells: Vec<Spell>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum WarPreference {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

impl WarPreference {
    #[must_use]
    pub fn is_opted_in(&self) -> bool {
        self == &Self::In
    }
    #[must_use]
    pub fn is_opted_out(&self) -> bool {
        self == &Self::Out
    }
}

impl ToString for WarPreference {
    fn to_string(&self) -> String {
        match self {
            Self::In => "in".to_string(),
            Self::Out => "out".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub name: String,
    pub stars: i32,
    pub value: i32,
    pub target: i32,
    pub info: String,
    pub completion_info: Option<String>,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub name: String,
    pub clan_level: i8,
    pub badge_urls: badge_urls::BadgeUrls,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Troop {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Village {
    #[serde(rename = "builderBase")]
    BuilderBase,

    #[serde(rename = "home")]
    HomeVillage,
}

impl Village {
    #[must_use]
    pub fn is_home(&self) -> bool {
        self == &Self::HomeVillage
    }
    #[must_use]
    pub fn is_builder_base(&self) -> bool {
        self == &Self::BuilderBase
    }
}

impl ToString for Village {
    fn to_string(&self) -> String {
        match self {
            Self::HomeVillage => "Home Village".to_string(),
            Self::BuilderBase => "Builder Base".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerToken {
    pub tag: String,
    pub token: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegendStatistics {
    pub legend_trophies: i32,
    pub previous_season: Option<season::PreviousSeasonData>,
    pub best_season: Option<season::BestSeasonData>,
    pub previous_versus_season: Option<season::PreviousVersusSeasonData>,
    pub best_versus_season: Option<season::BestVersusSeasonData>,
    pub current_season: season::CurrentSeasonData,
}

#[cfg(feature = "extra")]
impl Player {
    const HOME_TROOP_ORDER: [&'static str; 24] = [
        "Barbarian",
        "Archer",
        "Giant",
        "Goblin",
        "Wall Breaker",
        "Balloon",
        "Wizard",
        "Healer",
        "Dragon",
        "P.E.K.K.A",
        "Baby Dragon",
        "Miner",
        "Electro Dragon",
        "Yeti",
        "Dragon Rider",
        "Minion",
        "Hog Rider",
        "Valkyrie",
        "Golem",
        "Witch",
        "Lava Hound",
        "Bowler",
        "Ice Golem",
        "Headhunter",
    ];

    const SIEGE_MACHINE_ORDER: [&'static str; 6] = [
        "Wall Wrecker",
        "Battle Blimp",
        "Stone Slammer",
        "Siege Barracks",
        "Log Launcher",
        "Flame Flinger",
    ];

    const SUPER_TROOP_ORDER: [&'static str; 14] = [
        "Super Barbarian",
        "Super Archer",
        "Super Giant",
        "Sneaky Goblin",
        "Super Wall Breaker",
        "Rocket Balloon",
        "Super Wizard",
        "Super Dragon",
        "Inferno Dragon",
        "Super Minion",
        "Super Valkyrie",
        "Super Witch",
        "Ice Hound",
        "Super Bowler",
    ];

    const BUILDER_TROOP_ORDER: [&'static str; 11] = [
        "Raged Barbarian",
        "Sneaky Archer",
        "Boxer Giant",
        "Beta Minion",
        "Bomber",
        "Baby Dragon",
        "Cannon Cart",
        "Night Witch",
        "Drop Ship",
        "Super P.E.K.K.A",
        "Hog Glider",
    ];

    const SPELL_ORDER: [&'static str; 12] = [
        "Lightning Spell",
        "Healing Spell",
        "Rage Spell",
        "Jump Spell",
        "Freeze Spell",
        "Clone Spell",
        "Invisibility Spell",
        "Poison Spell",
        "Earthquake Spell",
        "Haste Spell",
        "Skeleton Spell",
        "Bat Spell",
    ];

    const HOME_HERO_ORDER: [&'static str; 4] =
        ["Barbarian King", "Archer Queen", "Grand Warden", "Royal Champion"];

    const HERO_ORDER: [&'static str; 5] =
        ["Barbarian King", "Archer Queen", "Grand Warden", "Royal Champion", "Battle Machine"];

    const HERO_PETS_ORDER: [&'static str; 4] =
        ["L.A.S.S.I", "Electro Owl", "Mighty Yak", "Unicorn"];
    #[allow(dead_code)]
    const ACHIEVEMENT_ORDER: [&'static str; 43] = [
        // Home Base
        "Keep Your Account Safe!",
        "Bigger & Better",
        "Discover New Troops",
        "Bigger Coffers",
        "Gold Grab",
        "Elixir Escapade",
        "Heroic Heist",
        "Well Seasoned",
        "Nice and Tidy",
        "Empire Builder",
        "Clan War Wealth",
        "Friend in Need",
        "Sharing is caring",
        "Siege Sharer",
        "War Hero",
        "War League Legend",
        "Games Champion",
        "Unbreakable",
        "Sweet Victory!",
        "Conqueror",
        "League All-Star",
        "Humiliator",
        "Not So Easy This Time",
        "Union Buster",
        "Bust This!",
        "Wall Buster",
        "Mortar Mauler",
        "X-Bow Exterminator",
        "Firefighter",
        "Anti-Artillery",
        "Shattered and Scattered",
        "Get those Goblins!",
        "Get those other Goblins!",
        "Dragon Slayer",
        "Superb Work",
        // Builder Base
        "Master Engineering",
        "Hidden Treasures",
        "High Gear",
        "Next Generation Model",
        "Un-Build It",
        "Champion Builder",
        // Clan Capital
        "Aggressive Capitalism",
        "Most Valuable Clanmate",
    ];

    pub fn game_link(&self) -> String {
        format!(
            "https://link.clashofclans.com/en?action=OpenPlayerProfile&tag={}",
            self.tag.replace('#', "")
        )
    }

    pub fn clash_of_stats_link(&self) -> String {
        format!("https://www.clashofstats.com/players/{}/summary", self.tag.replace('#', ""))
    }

    pub fn chocolate_clash_link(&self) -> String {
        format!("https://cc.chocolateclash.com/cc_n/member.php?tag={}", self.tag.replace('#', ""))
    }

    pub fn achievements(&self) -> Vec<Achievement> {
        Self::ACHIEVEMENT_ORDER
            .iter()
            .filter_map(|achievement_name| {
                self.achievements.iter().find(|achievement| &achievement.name == achievement_name)
            })
            .cloned()
            .collect()
    }

    pub fn get_achievement(&self, name: &str) -> Option<&Achievement> {
        self.achievements.iter().find(|achievement| achievement.name == name)
    }

    pub fn home_troops(&self) -> Vec<Troop> {
        Self::HOME_TROOP_ORDER
            .iter()
            .filter_map(|name| self.troops.iter().find(|troop| &troop.name == name))
            .cloned()
            .collect()
    }

    pub fn super_troops(&self) -> Vec<Troop> {
        Self::SUPER_TROOP_ORDER
            .iter()
            .filter_map(|name| self.troops.iter().find(|troop| &troop.name == name))
            .cloned()
            .collect()
    }

    pub fn builder_troops(&self) -> Vec<Troop> {
        Self::BUILDER_TROOP_ORDER
            .iter()
            .filter_map(|name| self.troops.iter().find(|troop| &troop.name == name))
            .cloned()
            .collect()
    }

    pub fn get_troop(&self, name: &str) -> Option<&Troop> {
        self.troops.iter().find(|troop| troop.name == name)
    }

    pub fn siege_machines(&self) -> Vec<Troop> {
        Self::SIEGE_MACHINE_ORDER
            .iter()
            .filter_map(|name| self.troops.iter().find(|troop| &troop.name == name))
            .cloned()
            .collect()
    }

    pub fn home_heroes(&self) -> Vec<Hero> {
        Self::HOME_HERO_ORDER
            .iter()
            .filter_map(|name| self.heroes.iter().find(|hero| &hero.name == name))
            .cloned()
            .collect()
    }

    pub fn heroes(&self) -> Vec<Hero> {
        // map it to the order of Self::HERO_ORDER
        Self::HERO_ORDER
            .iter()
            .filter_map(|name| self.heroes.iter().find(|hero| &hero.name == name).cloned())
            .collect()
    }

    pub fn hero_pets(&self) -> Vec<Troop> {
        Self::HERO_PETS_ORDER
            .iter()
            .filter_map(|name| self.troops.iter().find(|troop| &troop.name == name))
            .cloned()
            .collect()
    }

    pub fn get_hero(&self, name: &str) -> Option<&Hero> {
        self.heroes.iter().find(|hero| hero.name == name)
    }

    pub fn get_pet(&self, name: &str) -> Option<&Troop> {
        self.troops.iter().find(|troop| troop.name == name)
    }

    pub fn spells(&self) -> Vec<Spell> {
        Self::SPELL_ORDER
            .iter()
            .filter_map(|name| self.spells.iter().find(|spell| &spell.name == name).cloned())
            .collect()
    }

    pub fn get_spell(&self, name: &str) -> Option<&Spell> {
        self.spells.iter().find(|spell| spell.name == name)
    }
}
