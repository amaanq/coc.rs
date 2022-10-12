# coc.rs
A Clash of Clans API wrapper written in rust!

# Key feature
- Asynchronous code
- Entire coverage of [Clash of clans API](https://developer.clashofclans.com)
- Email and Password Login
- Multiple Accounts Login to handle concurrent requests
- API Events!
- [Clash of Stats](https://www.clashofstats.com/) support

Getting Started
================

## Installing

Add the version from [here](https://crates.io/crates/coc-rs) in your Cargo.toml
<br/>

```toml
[dependencies]
coc-rs = "x.x.x"
```

Alternatively with `cargo add`

```shell
$ cargo add coc-rs
```

## Quick Example

```rust
#[tokio::main]
async fn main() {
    let credentials = CredentialsBuilder::new()
        .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
        .build();
    let client = Client::new(credentials).await;

    let player = client.get_player("#2PP".to_string()).await.unwrap();

    println!("Player: {:?}", player);
}
```

### How to Handle Errors
```rust
#[tokio::main]
async fn main() {
    let credentials = CredentialsBuilder::new()
        .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
        .build();
    let client = Client::new(credentials).await;

    let clan = match client.get_clan("#IncorrectTag".to_string()).await {
        Ok(c) => {
            println!("Clan: {:?}", clan);
        }
        Err(err) => {
            match err {
                APIError::BadRequest(err) => {} // caused when a BadRequest is made, such as invalid url parameters 
                APIError::RequestFailed(err) => {} // Request never made it to the API
                APIError::BadResponse(msg, code) => {
                    match code {
                        StatusCode::NOT_FOUND => {
                            println!("Player Not found")
                        },
                        _ => {
                            //There are other status code that the api can return in case of error.
                        }
                    }
                } // this is common error you will face and will be expected to handle it almost everytime
                APIError::InvalidParameters(err) => {}
            }
        }
    };
}
```

### Basic Events

First a struct should be created that will implement the trait `EventHandler`

```rust
struct Handler;

#[async_trait]
impl events::EventHandler for S {
    /// Next we bring the player method in scope, and define the behaviour
    async fn player(&self, _old_player: Option<player::Player>, _new_player: player::Player) {
        println!("Player change detected!")
    }

    ///  to handle errors in the events task we need a separate error handler
    async fn handle_error(
        &self,
        _error: APIError,
        _tag: Option<String>,
        _event_type: EventType,
    ) {
        println!("Houston we have a problem!")
    }
}
```

Next in the main function, we will create the main function, login and add the Player and clan tags we want to keep
pulling the data from API.

```rust
#[tokio::test]
async fn main() {
    //...
    /// see above example on how to create a client

    let task = tokio::spawn(async move {
        /// staring the API events in a separate thread
        let mut event_builder = events::EventsListenerBuilder::new(client);
        event_builder.add_player("#2PP").add_players(vec!["#CVJLQOLR"])
            .build(Handler) /// Building the eventListener struct 
            .start() /// starting the continuous polling of the clan/player/current_war endpoints
            .await;
    });
    task.await.unwrap();
}
```

### Features

To Enable `cos` feature, add this to your `Cargo.toml`

```toml
[dependencies]
coc-rs = { version = "x.x.x", features = ["cos"] }
```

- Alternately with `cargo add`

```shell
$ cargo add coc-rs --features cos
```

*Note: Each endpoint has a different cache refresh time. Each event will be fired at the exact time of new cache data in
the API.*

#### Possible Error Code

400 -> BadRequestException <br/>
403 -> AuthException <br/>
404 -> NotFoundException<br/>
429 -> RateLimitException <br/>
503 -> MaintenanceException<br/>

#### Note

`src/lib.rs` contains examples (in the form of tests) for every endpoint in a bit more detail.

# Contributing
Contributing is fantastic and much welcomed! If you have an issue, feel free to open an issue and start working on it.

# Disclaimer
This content is not affiliated with, endorsed, sponsored, or specifically
approved by Supercell and Supercell is not responsible for it.
For more information see [Supercell's Fan Content Policy](https://www.supercell.com/fan-content-policy).
