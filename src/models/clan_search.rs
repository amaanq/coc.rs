use super::location::Local;

#[derive(Debug, Default)]
pub struct ClanSearchOptionsBuilder {
    options: ClanSearchOptions,
}

#[derive(Debug, Default)]
pub struct ClanSearchOptions {
    name: Option<String>,
    war_frequency: Option<String>,
    location_id: Option<i32>,
    min_members: Option<i32>,
    max_members: Option<i32>,
    min_clan_points: Option<i32>,
    min_clan_level: Option<i8>,
    limit: Option<i32>,
    after: Option<String>,
    before: Option<String>,
    label_ids: Option<Vec<String>>,

    pub(crate) items: Vec<(String, String)>,
}

// implement iter for ClanSearchOptions by iterating over every field

impl ClanSearchOptionsBuilder {
    pub fn new() -> Self {
        let mut s = Self {
            options: ClanSearchOptions::default(),
        };
        // empty vec of (String, String) with size of 11
        s.options.items = vec![(String::new(), String::new()); 11];
        s
    }
    pub fn name(mut self, name: String) -> Self {
        self.options.name = Some(name.clone());
        self.options.items[0] = ("name".to_string(), name);
        self
    }
    pub fn war_frequency(mut self, war_frequency: String) -> Self {
        self.options.war_frequency = Some(war_frequency.clone());
        self.options.items[1] = ("warFrequency".to_string(), war_frequency);
        self
    }
    pub fn location_id(mut self, location_id: Local) -> Self {
        let i = location_id as i32;
        self.options.location_id = Some(i);
        self.options.items[2] = ("locationId".to_string(), (i).to_string());
        self
    }
    pub fn min_members(mut self, min_members: i32) -> Self {
        if min_members >= 2 {
            self.options.min_members = Some(min_members);
            self.options.items[3] = ("minMembers".to_string(), min_members.to_string());
        }
        self
    }
    pub fn max_members(mut self, max_members: i32) -> Self {
        if max_members <= 50 {
            self.options.max_members = Some(max_members);
            self.options.items[4] = ("maxMembers".to_string(), max_members.to_string());
        }
        self
    }
    pub fn min_clan_points(mut self, min_clan_points: i32) -> Self {
        self.options.min_clan_points = Some(min_clan_points);
        self.options.items[5] = ("minClanPoints".to_string(), min_clan_points.to_string());
        self
    }
    pub fn min_clan_level(mut self, min_clan_level: i8) -> Self {
        if min_clan_level >= 2 {
            self.options.min_clan_level = Some(min_clan_level);
            self.options.items[6] = ("minClanLevel".to_string(), min_clan_level.to_string());
        }
        self
    }
    pub fn limit(mut self, limit: i32) -> Self {
        self.options.limit = Some(limit);
        self.options.items[7] = ("limit".to_string(), limit.to_string());
        self
    }
    pub fn after(mut self, after: String) -> Self {
        self.options.after = Some(after.clone());
        self.options.items[8] = ("after".to_string(), after);
        self
    }
    pub fn before(mut self, before: String) -> Self {
        self.options.before = Some(before.clone());
        self.options.items[9] = ("before".to_string(), before);
        self
    }
    pub fn label_ids(mut self, label_ids: Vec<String>) -> Self {
        self.options.label_ids = Some(label_ids.clone());
        self.options.items[10] = ("labelIds".to_string(), label_ids.join(","));
        self
    }
    pub fn build(self) -> ClanSearchOptions {
        self.options
    }
}
