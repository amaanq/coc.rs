use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, MutexGuard};

use crate::api::{APIError, Client};
use crate::models::clan::Clan;
use crate::models::current_war::WarClan;
use crate::models::player::Player;
use crate::models::player::WarPreference::In;

#[async_trait]
pub trait EventHandler {
    async fn player(&self, old_player: Option<Player>, new_player: Player) {}
    async fn clan(&self, old_clan: Option<Clan>, new_clan: Clan) {}
    async fn war(&self, old_war: Option<WarClan>, new_war: WarClan) {}
}

#[derive(Debug)]
pub struct EventsListenerBuilder<'a> {
    event_type: EventType,
    client: &'a Client,
}

const TEMP_COUNT: i8 = 0;

#[derive(Debug)]
pub enum EventType {
    Player(String, Instant, Option<Player>),
    Clan(String, Instant, Option<Clan>),
    War(String, Instant, Option<WarClan>),
    None,
}

impl<'a> EventsListenerBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        return EventsListenerBuilder {
            event_type: EventType::None,
            client,
        };
    }

    pub async fn add_clan(&mut self, tag: String) {
        self.event_type = EventType::Clan(tag, Instant::now(), None)
    }

    pub async fn add_player(&mut self, tag: String) {
        self.event_type = EventType::Player(tag, Instant::now(), None)
    }

    pub async fn add_war(&mut self, tag: String) {
        self.event_type = EventType::War(tag, Instant::now(), None)
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
    event_type: EventType,
    client: &'a Client,
    handler: T,
    last_time_fired: Instant,
}

impl<'a, T> EventsListener<'a, T>
    where T: EventHandler + Sync + Send
{
    pub async fn init(&mut self) -> ! {
        loop {
            match self.fire_events().await {
                Ok(b) => {
                    if b == true {
                        println!("Got successfully")
                    }
                }
                Err(_) => {
                    println!("Error in Events");
                    break;
                }
            };
        }
    }
    async fn fire_events(&mut self) -> Result<bool, ()> {
        fn should_fire_again(duration: Duration, minutes: u64) -> bool {
            duration.as_secs() >= minutes
        }
        match &self.event_type {
            EventType::Player(tag, last_fired, old) => {
                let option = Instant::now().checked_duration_since(*last_fired);

                match option {
                    None => {}
                    Some(q) => {
                        if should_fire_again(q, 30) {
                            let new = self.client.get_player(tag.to_owned()).await.unwrap();
                            self.handler.player(old.clone(), new.clone()).await; // invoking the handler function the user defined
                            self.event_type = EventType::Player(tag.to_owned(), Instant::now(), Some(new));
                            return Ok(true);
                        }
                    }
                };
            }
            EventType::Clan(tag, now, old) => {
                todo!()
            }
            EventType::War(tag, now, old) => {
                todo!()
            }
            EventType::None => {
                return Err(());
            }
        };
        Ok(false)
    }
}