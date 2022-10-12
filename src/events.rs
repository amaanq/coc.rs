use std::time::{Duration, Instant};

use async_trait::async_trait;

use crate::{
    api::{APIError, Client},
    models::*,
    war::War,
};

#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler {
    async fn player(&self, old_player: Option<player::Player>, new_player: player::Player) {}
    async fn clan(&self, old_clan: Option<clan::Clan>, new_clan: clan::Clan) {}
    async fn war(&self, old_war: Option<War>, new_war: War) {}
    async fn handle_error(&self, error: APIError, tag: String, event_type: EventType);
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
            EventType::Player(tag, _, _) => write!(f, "PlayerEvent({})", tag),
            EventType::Clan(tag, _, _) => write!(f, "ClanEvent({})", tag),
            EventType::War(tag, _, _) => write!(f, "WarEvent({})", tag),
        }
    }
}

impl EventsListenerBuilder {
    pub fn new(client: Client) -> Self {
        EventsListenerBuilder { event_type: vec![], client }
    }

    pub fn add_clan(mut self, tag: impl ToString) -> Self {
        self.event_type.push(EventType::Clan(tag.to_string(), Instant::now(), None));
        self
    }

    pub fn add_player(mut self, tag: impl ToString) -> Self {
        self.event_type.push(EventType::Player(tag.to_string(), Instant::now(), None));
        self
    }

    pub fn add_war(mut self, tag: impl ToString) -> Self {
        self.event_type.push(EventType::War(tag.to_string(), Instant::now(), None));
        self
    }

    pub fn add_clans(mut self, tags: Vec<impl ToString>) -> Self {
        // since add_clan takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::Clan(tag.to_string(), Instant::now(), None));
        }
        self
    }

    pub fn add_players(mut self, tags: Vec<impl ToString>) -> Self {
        // since add_player takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::Player(tag.to_string(), Instant::now(), None));
        }

        self
    }

    pub fn add_wars(&mut self, tags: Vec<impl ToString>) -> &mut Self {
        // since add_war takes self by value, we have to use a for loop
        for tag in tags {
            self.event_type.push(EventType::War(tag.to_string(), Instant::now(), None));
        }
        self
    }

    pub fn build<T: EventHandler>(self, handler: T) -> EventsListener<T>
    where
        T: EventHandler + Sync + Send,
    {
        EventsListener {
            event_type: self.event_type,
            client: self.client,
            handler,
            last_time_fired: Instant::now(),
        }
    }
}

pub struct EventsListener<T>
where
    T: EventHandler + Sync + Send,
{
    event_type: Vec<EventType>,
    client: Client,
    handler: T,
    #[allow(dead_code)]
    last_time_fired: Instant,
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
    pub async fn start(mut self, duration: Option<Duration>) -> Result<(), EventsError> {
        if let Some(duration) = duration {
            let start = Instant::now();
            while start.elapsed() < duration {
                if let Err(e) = self.fire_events().await {
                    self.event_type.remove(e.index);
                    self.handler.handle_error(e.api_error, e.tag, e.event_type).await;
                };
            }
        } else {
            loop {
                if let Err(e) = self.fire_events().await {
                    self.event_type.remove(e.index);
                    self.handler.handle_error(e.api_error, e.tag, e.event_type).await;
                };
            }
        }

        Ok(())
    }

    async fn fire_events(&mut self) -> Result<bool, EventsError> {
        #[inline(always)]
        fn should_fire_again(duration: Duration, seconds: u64) -> bool {
            duration.as_secs() >= seconds
        }

        for (i, event) in self.event_type.iter().enumerate() {
            match event {
                EventType::Player(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if should_fire_again(duration, 10) {
                            return match self.client.get_player(tag).await {
                                Ok(new) => {
                                    self.handler.player(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] = EventType::Player(
                                        tag.to_owned(),
                                        Instant::now(),
                                        Some(new),
                                    );
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.to_owned(),
                                    event_type: event.clone(),
                                    index: i,
                                }),
                            };
                        }
                    };
                }
                EventType::Clan(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if should_fire_again(duration, 10) {
                            return match self.client.get_clan(tag).await {
                                Ok(new) => {
                                    self.handler.clan(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] =
                                        EventType::Clan(tag.to_owned(), Instant::now(), Some(new));
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.to_owned(),
                                    event_type: event.clone(),
                                    index: i,
                                }),
                            };
                        }
                    };
                }
                EventType::War(tag, last_fired, old) => {
                    if let Some(duration) = Instant::now().checked_duration_since(*last_fired) {
                        if should_fire_again(duration, 60 * 10) {
                            return match self.client.get_current_war(tag).await {
                                Ok(new) => {
                                    self.handler.war(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                    self.event_type[i] =
                                        EventType::War(tag.to_owned(), Instant::now(), Some(new));
                                    Ok(true)
                                }
                                Err(err) => Err(EventsError {
                                    api_error: err,
                                    tag: tag.to_owned(),
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
