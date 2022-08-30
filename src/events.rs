use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, MutexGuard};

use crate::api::{APIError, Client};
use crate::models::clan::Clan;
use crate::models::player::Player;
use crate::models::player::WarPreference::In;
use crate::war::War;

#[async_trait]
pub trait EventHandler {
    async fn player(&self, old_player: Option<Player>, new_player: Player) {}
    async fn clan(&self, old_clan: Option<Clan>, new_clan: Clan) {}
    async fn war(&self, old_war: Option<War>, new_war: War) {}
    async fn handle_error(&self, error: APIError, tag: Option<String>, event_type: EventType);
}

#[derive(Debug)]
pub struct EventsListenerBuilder<'a> {
    event_type: Vec<EventType>,
    client: &'a Client,
}

#[derive(Debug, Clone)]
pub enum EventType {
    Player(String, Instant, Option<Player>),
    Clan(String, Instant, Option<Clan>),
    War(String, Instant, Option<War>),
    None,
}

impl<'a> EventsListenerBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        return EventsListenerBuilder {
            event_type: vec![],
            client,
        };
    }

    pub async fn add_clan(&mut self, tag: String) {
        self.event_type.push(EventType::Clan(tag, Instant::now(), None))
    }

    pub async fn add_clans(mut self, tags: Vec<String>) -> EventsListenerBuilder<'a> {
        for x in tags {
            self.add_clan(x).await;
        }
        self
    }

    pub async fn add_player(&mut self, tag: String) {
        self.event_type.push(EventType::Player(tag, Instant::now(), None));
    }

    pub async fn add_players(mut self, tags: Vec<String>) -> EventsListenerBuilder<'a> {
        for x in tags {
            self.add_player(x).await;
        }
        self
    }


    pub async fn add_war(&mut self, tag: String) {
        self.event_type.push(EventType::War(tag, Instant::now(), None));
    }

    pub async fn add_wars(mut self, tags: Vec<String>) -> EventsListenerBuilder<'a> {
        for x in tags {
            self.add_war(x).await;
        }
        self
    }


    pub fn build<T: EventHandler>(self, handler: T) -> EventsListener<'a, T>
        where T: EventHandler + Sync + Send
    {
        EventsListener {
            event_type: self.event_type,
            client: self.client,
            handler,
            last_time_fired: Instant::now(),
        }
    }
}

pub struct EventsListener<'a, T>
    where T: EventHandler + Sync + Send
{
    event_type: Vec<EventType>,
    client: &'a Client,
    handler: T,
    last_time_fired: Instant,
}

struct EventsError {
    api_error: APIError,
    tag: Option<String>,
    event_type: EventType,
    index: usize,
}
impl<'a, T> EventsListener<'a, T>
    where T: EventHandler + Sync + Send
{
    pub async fn init(mut self) {
        loop {
            match self.fire_events().await {
                Ok(_) => {}
                Err(err) => {
                    println!("Error in Events");
                    self.event_type.remove(err.index);
                    self.handler.handle_error(err.api_error, err.tag, err.event_type).await;
                }
            };
        }
    }
    async fn fire_events(&mut self) -> Result<bool, EventsError> {
        fn should_fire_again(duration: Duration, seconds: u64) -> bool {
            duration.as_secs() >= seconds
        }
        for (i, e) in self.event_type.iter().enumerate() {
            match e {
                EventType::Player(tag, last_fired, old) => {
                    let option = Instant::now().checked_duration_since(*last_fired);

                    match option {
                        None => {}
                        Some(q) => {
                            if should_fire_again(q, 10) {
                                return match self.client.get_player(tag.to_owned()).await {
                                    Ok(new) => {
                                        self.handler.player(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                        self.event_type[i] = EventType::Player(tag.to_owned(), Instant::now(), Some(new));
                                        Ok(true)
                                    }
                                    Err(err) => {
                                        Err(
                                            EventsError {
                                                api_error: err,
                                                tag: Some(tag.to_owned()),
                                                event_type: e.clone(),
                                                index: i,
                                            }
                                        )
                                    }
                                };
                            }
                        }
                    };
                }
                EventType::Clan(tag, last_fired, old) => {
                    let option = Instant::now().checked_duration_since(*last_fired);

                    match option {
                        None => {}
                        Some(q) => {
                            if should_fire_again(q, 10) {
                                return match self.client.get_clan(tag.to_owned()).await {
                                    Ok(new) => {
                                        self.handler.clan(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                        self.event_type[i] = EventType::Clan(tag.to_owned(), Instant::now(), Some(new));
                                        Ok(true)
                                    }
                                    Err(err) => {
                                        Err(
                                            EventsError {
                                                api_error: err,
                                                tag: Some(tag.to_owned()),
                                                event_type: e.clone(),
                                                index: i,
                                            }
                                        )
                                    }
                                };
                            }
                        }
                    };
                }
                EventType::War(tag, last_fired, old) => {
                    let option = Instant::now().checked_duration_since(*last_fired);

                    match option {
                        None => {}
                        Some(q) => {
                            if should_fire_again(q, 60 * 10) {
                                return match self.client.get_current_war(tag.to_owned()).await {
                                    Ok(new) => {
                                        self.handler.war(old.clone(), new.clone()).await; // invoking the handler function the user defined
                                        self.event_type[i] = EventType::War(tag.to_owned(), Instant::now(), Some(new));
                                        Ok(true)
                                    }
                                    Err(err) => {
                                        Err(
                                            EventsError {
                                                api_error: err,
                                                tag: Some(tag.to_owned()),
                                                event_type: e.clone(),
                                                index: i,
                                            }
                                        )
                                    }
                                };
                            }
                        }
                    };
                }
                EventType::None => {
                    return Err(EventsError {
                        api_error: APIError::EventFailure("[UNREACHABLE] NO EVENT TYPE WAS SPECIFIED".to_owned()),
                        tag: None,
                        event_type: EventType::None,
                        index: i,
                    })
                }
            }
        };
        Ok(false)
    }
}