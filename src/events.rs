use std::ops::Add;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, MutexGuard};

use crate::api::Client;
use crate::models::clan::Clan;
use crate::models::current_war::WarClan;
use crate::models::player::Player;

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
        where T: EventHandler + Sync + Send
    {
        EventsListener {
            event_type: self.event_type,
            client: self.client,
            handler,
            last_time_fired: Utc::now(),
        }
    }
}

pub struct EventsListener<'a, T>
    where T: EventHandler + Sync + Send
{
    event_type: EventType,
    client: &'a Client,
    handler: T,
    last_time_fired: chrono::DateTime<Utc>,
}

impl<'a, T> EventsListener<'a, T>
    where T: EventHandler + Sync + Send
{
    pub async fn init(&mut self) -> ! {
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
    async fn fire_events(&mut self) -> Result<(), ()> {
        match &self.event_type {
            EventType::Player(tag, now, old) => {
                let new = self.client.get_player(tag.to_owned()).await.unwrap();
                match old {
                    None => { println!("NONE") } //debug info to remove
                    Some(_) => {}
                }
                self.handler.player(old.clone(), new.clone()).await;
                self.event_type = EventType::Player(tag.to_owned(), Utc::now(), Some(new))
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