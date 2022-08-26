use std::ops::Add;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::{Mutex, MutexGuard};

use crate::api::Client;
use crate::models::clan::Clan;
use crate::models::current_war::WarClan;
use crate::models::player::Player;

#[async_trait]
pub trait EventHandler {
    fn player(old_player: Option<Player>, new_player: Player);
}

#[derive(Debug)]
pub struct EventsListenerBuilder<'a> {
    event_type: EventType,
    client: &'a Client,
}

#[derive(Debug)]
pub enum EventType {
    Player(String, chrono::DateTime<Utc>, Option<Player>),
    Clan(String, chrono::DateTime<Utc>, Option<Clan>),
    War(String, chrono::DateTime<Utc>, Option<WarClan>),
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
        self.event_type = EventType::Clan(tag, Utc::now(), None)
    }

    pub async fn add_player(&mut self, tag: String) {
        self.event_type = EventType::Player(tag, Utc::now(), None)
    }

    pub async fn add_war(&mut self, tag: String) {
        self.event_type = EventType::War(tag, Utc::now(), None)
    }

    pub fn build<T: EventHandler>(self, handler: T) -> EventsListener<'a, T>
        where T: EventHandler
    {
        EventsListener {
            event_type: self.event_type,
            client: self.client,
            handler,
            last_time_fired: chrono::offset::Utc::now(),
        }
    }
}

pub struct EventsListener<'a, T: EventHandler>
    where T: EventHandler
{
    event_type: EventType,
    client: &'a Client,
    handler: T,
    last_time_fired: chrono::DateTime<Utc>,
}

impl<'a, T: EventHandler> EventsListener<'a, T> {
    pub async fn init(&self) {
        loop {
            match self.fire_events().await {
                Ok(_) => {
                    println!("Got successfully")
                }
                Err(_) => {
                    println!("Not success")
                }
            };
        }
    }
    async fn fire_events(&self) -> Result<(), ()> {
        match &self.event_type {
            EventType::Player(tag, now, old) => {
                println!("new event")
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
        Ok(())
    }
}