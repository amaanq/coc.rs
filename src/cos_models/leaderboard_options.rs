#[cfg(feature = "cos")]
pub mod cos_options {
    use crate::location;

    pub struct Options {
        /// Cannot be used with the "legends" method, only with the "clan" or "player" method.
        pub location: location::Local,
        /// Only to be used with "player" methods or methods that return crate::PlayerRanking
        pub level: Level,
        /// All methods take this
        pub page: i32,
    }

    pub struct OptionsBuilder {
        pub options: Options,
    }

    impl Options {
        pub fn new(location: location::Local, level: Level, page: i32) -> Self {
            Options { location, level, page: page.clamp(0, 100000) }
        }

        pub fn builder() -> OptionsBuilder {
            OptionsBuilder {
                options: Options {
                    location: location::Local::default(),
                    level: Level::default(),
                    page: 0,
                },
            }
        }

        pub(crate) fn build_for_clan(self) -> Vec<(String, String)> {
            match self.location {
                location::Local::None => vec![
                    ("location".to_string(), "global".to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
                _ => vec![
                    ("location".to_string(), (self.location as i32).to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
            }
        }

        pub(crate) fn build_for_player(self) -> Vec<(String, String)> {
            match self.location {
                location::Local::None => vec![
                    ("location".to_string(), "global".to_string()),
                    ("level".to_string(), (self.level as i32).to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
                _ => vec![
                    ("location".to_string(), (self.location as i32).to_string()),
                    ("level".to_string(), (self.level as i32).to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
            }
        }

        pub(crate) fn build_for_legends(self) -> Vec<(String, String)> {
            vec![("page".to_string(), self.page.to_string())]
        }

        pub(crate) fn build_for_builder(self) -> Vec<(String, String)> {
            match self.location {
                location::Local::None => vec![
                    ("location".to_string(), "global".to_string()),
                    ("level".to_string(), (self.level as i32).min(9).to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
                _ => vec![
                    ("location".to_string(), (self.location as i32).to_string()),
                    ("level".to_string(), (self.level as i32).min(9).to_string()),
                    ("page".to_string(), self.page.to_string()),
                ],
            }
        }
    }

    impl OptionsBuilder {
        pub fn location(mut self, location: location::Local) -> Self {
            self.options.location = location;
            self
        }

        pub fn level(mut self, level: i32) -> Self {
            self.options.level = match level {
                1 => Level::One,
                2 => Level::Two,
                3 => Level::Three,
                4 => Level::Four,
                5 => Level::Five,
                6 => Level::Six,
                7 => Level::Seven,
                8 => Level::Eight,
                9 => Level::Nine,
                10 => Level::Ten,
                11 => Level::Eleven,
                12 => Level::Twelve,
                13 => Level::Thirteen,
                14 => Level::Fourteen,
                _ => Level::default(),
            };
            self
        }

        /// minimum of 1
        pub fn page(mut self, mut page: i32) -> Self {
            if page <= 0 {
                page = 1;
            }
            self.options.page = (page - 1).clamp(0, 100000);
            self
        }

        pub fn build(self) -> Options {
            self.options
        }
    }

    pub enum Level {
        /// All Town Halls
        None = 0,
        /// Town Hall 1
        One = 1,
        /// Town Hall 2
        Two = 2,
        /// Town Hall 3
        Three = 3,
        /// Town Hall 4
        Four = 4,
        /// Town Hall 5
        Five = 5,
        /// Town Hall 6
        Six = 6,
        /// Town Hall 7
        Seven = 7,
        /// Town Hall 8
        Eight = 8,
        /// Town Hall 9
        Nine = 9,
        /// Town Hall 10
        Ten = 10,
        /// Town Hall 11
        Eleven = 11,
        /// Town Hall 12
        Twelve = 12,
        /// Town Hall 13
        Thirteen = 13,
        /// Town Hall 14
        Fourteen = 14,
    }

    impl Default for Level {
        fn default() -> Self {
            Level::None
        }
    }
}
