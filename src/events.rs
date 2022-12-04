use std::time::{Duration, Instant};

use async_trait::async_trait;

use crate::{
    api::Client,
    error::APIError,
    models::{clan, player},
    war::War,
};

#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler {
    async fn player(&self, old_player: Option<player::Player>, new_player: player::Player) {}
    async fn clan(&self, old_clan: Option<clan::Clan>, new_clan: clan::Clan) {}
    async fn war(&self, old_war: Option<War>, new_war: War) {}
    async fn on_error(&self, error: APIError, tag: String, event_type: EventType);
}

#[derive(Debug)]
pub struct EventsListenerBuilder {
    event_type: Vec<EventType>,
    client: Client,
}

#[derive(Debug, Clone)]
pub enum EventType {
    Player(String, Instant, Option<player::Player>),
    Clan(String, Instant, Option<clan::Clan>),
    War(String, Instant, Option<War>),
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(tag, _, _) => write!(f, "PlayerEvent({tag})"),
            Self::Clan(tag, _, _) => write!(f, "ClanEvent({tag})"),
            Self::War(tag, _, _) => write!(f, "WarEvent({tag})"),
        }
    }
}

impl EventsListenerBuilder {
    #[must_use]
    pub const fn new(client: Client) -> Self {
        Self { event_type: vec![], client }
    }

    #[must_use]
    pub fn add_clan(mut self, tag: &str) -> Self {
        self.event_type.push(EventType::Clan(tag.to_string(), Instant::now(), None));
        self
    }

    #[must_use]
    pub fn add_player(mut self, tag: &str) -> Self {
        self.event_type.push(EventType::Player(tag.to_string(), Instant::now(), None));
        self
    }

    #[must_use]
    pub fn add_war(mut self, tag: &str) -> Self {
        self.event_type.push(EventType::War(tag.to_string(), Instant::now(), None));
        self
    }

    #[must_use]
    pub fn add_clans(mut self, tags: Vec<impl ToString>) -> Self {
        // since add_clan takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::Clan(tag.to_string(), Instant::now(), None));
        }
        self
    }

    #[must_use]
    pub fn add_players(mut self, tags: Vec<impl ToString>) -> Self {
        // since add_player takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::Player(tag.to_string(), Instant::now(), None));
        }

        self
    }

    #[must_use]
    pub fn add_wars(&mut self, tags: Vec<impl ToString>) -> &mut Self {
        // since add_war takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::War(tag.to_string(), Instant::now(), None));
        }
        self
    }

    pub fn build<T>(self, handler: T) -> EventsListener<T>
    where
        T: EventHandler + Sync + Send,
    {
        EventsListener { event_type: self.event_type, client: self.client, handler }
    }
}

pub struct EventsListener<T>
where
    T: EventHandler + Sync + Send,
{
    event_type: Vec<EventType>,
    client: Client,
    handler: T,
}

pub struct EventsError {
    api_error: APIError,
    tag: String,
    event_type: EventType,
    index: usize,
}

impl std::fmt::Display for EventsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while handling event `{}`: [{}]", self.event_type, self.api_error)
    }
}

impl std::fmt::Debug for EventsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while handling event `{}`: [{}]", self.event_type, self.api_error)
    }
}

impl std::error::Error for EventsError {}

impl<T> EventsListener<T>
where
    T: EventHandler + Sync + Send,
{
    /// Start the events listener, note that if duration is None, it will run forever
    ///
    /// # Errors
    ///
    /// This function will return an error if [`Client::get_player`], [`Client::get_clan`], or [`Client::get_current_war`] fails
    pub async fn start(mut self, duration: Option<Duration>) -> Result<(), EventsError> {
        if let Some(duration) = duration {
            let start = Instant::now();
            while start.elapsed() < duration {
                if let Err(e) = self.fire_events().await {
                    self.event_type.remove(e.index);
                    self.handler.on_error(e.api_error, e.tag, e.event_type).await;
                };
            }
        } else {
            loop {
                if let Err(e) = self.fire_events().await {
                    self.event_type.remove(e.index);
                    self.handler.on_error(e.api_error, e.tag, e.event_type).await;
                };
            }
        }

        Ok(())
    }

    #[inline(always)]
    const fn should_fire_again(duration: Duration, seconds: u64) -> bool {
        duration.as_secs() >= seconds
    }

    /// Returns true if a new event was fired for any [`EventType`]
    ///
    /// # Errors
    ///
    /// This function will return an error if [`crate::api::Client::get_player`] [`crate::api::Client::get_clan`], or [`crate::api::Client::get_current_war`] fails
    async fn fire_events(&mut self) -> Result<bool, EventsError> {
        for (i, event) in self.event_type.iter().enumerate() {
            match event {
                EventType::Player(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if Self::should_fire_again(duration, 10) {
                            return match self.client.get_player(tag).await {
                                Ok(new) => {
                                    self.handler.player(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] =
                                        EventType::Player(tag.clone(), Instant::now(), Some(new));
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.clone(),
                                    event_type: event.clone(),
                                    index: i,
                                }),
                            };
                        }
                    };
                }
                EventType::Clan(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if Self::should_fire_again(duration, 10) {
                            return match self.client.get_clan(tag).await {
                                Ok(new) => {
                                    self.handler.clan(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] =
                                        EventType::Clan(tag.clone(), Instant::now(), Some(new));
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.clone(),
                                    event_type: event.clone(),
                                    index: i,
                                }),
                            };
                        }
                    };
                }
                EventType::War(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if Self::should_fire_again(duration, 60 * 10) {
                            return match self.client.get_current_war(tag).await {
                                Ok(new) => {
                                    self.handler.war(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] =
                                        EventType::War(tag.clone(), Instant::now(), Some(new));
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.clone(),
                                    event_type: event.clone(),
                                    index: i,
                                }),
                            };
                        }
                    };
                }
            }
        }

        Ok(false)
    }
}
