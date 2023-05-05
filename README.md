# coc.rs

A Clash of Clans API wrapper written in Rust!

## Key Features

- Asynchronous code
- Entire coverage of [Clash of Clans API](https://developer.clashofclans.com)
- Email and password Login
- Ability to login with multiple accounts to handle many concurrent requests
- API Events to track changes
- [Clash of Stats](https://www.clashofstats.com/) support

## Getting Started

### Installing

Add the version from [here](https://crates.io/crates/coc-rs) in your Cargo.toml

```toml
[dependencies]
coc-rs = "0.8.1"
```

Or with `cargo add`

```sh
cargo add coc-rs --features=all
```

### Quick Example

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

### Error Handling

```rust
#[tokio::main]
async fn main() {
    let credentials = CredentialsBuilder::new()
        .add_credential(env::var("username").unwrap(), env::var("password").unwrap())
        .build();
    let client = Client::new(credentials).await;

    let clan = match client.get_clan("#InvalidTag".to_string()).await {
        Ok(clan) => {
            println!("Clan: {:?}", clan);
        }
        Err(err) => {
            match err {
                APIError::ClientNotReady => {}, // API login hasn't been initialized yet, try not to request with milliseconds of initializing a client
                APIError::FailedGetIP(err) => {}, // A request is made to api.ipify.org to set your IP dynamically when making keys, ensure this url isn't blocked
                APIError::LoginFailed(err) => {}, // Failed to login to a Clash of Stats account
                APIError::RequestFailed(err) => {}, // Request never made it to the API
                APIError::InvalidHeader(err) => {}, // you should not get this
                APIError::BadUrl(err) => {}, // you should also not get this
                APIError::BadParameters => {}, // bad input parameters for endpoints that have this
                APIError::AccessDenied => {}, // ip changed? or accessing something you shouldn't...
                APIError::NotFound => {}, // bad input "tags" or banned players result in this
                APIError::RequestThrottled => {}, // slow down!
                APIError::UnknownError => {}, // 🤨
                APIError::InMaintenance => {}, // doofus wait until it's over!
                APIError::BadResponse(err, err_code) => {}, // Catch-all error for those that don't fall in any of the above
                APIError::InvalidParameters(err) => {}, // I caught your parameter mistake, not the API!
                APIError::InvalidTag(err) => {}, // malformed tag
                APIError::EventFailure(err) => {}, // ? maybe I should remove this..
            }
        }
    };
}
```

### Basic Events

First a struct should be created that will implement the trait `EventHandler`,
this is similar to how the serenity discord library does event handling.

```rust
struct Handler;

#[async_trait]
impl events::EventHandler for S {
    /// Next we bring the player method in scope, and define the behaviour
    async fn player(&self, _old_player: Option<player::Player>, _new_player: player::Player) {
        println!("Player change detected!")
    }

    /// To handle errors in the events task we need a separate error handler
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

Next in the main function, we will create the main function,
login and add the Player and clan tags we want to keep pulling the data from API.

```rust
#[tokio::test]
async fn main() {
    //...
    /// see above example on how to create a client

    let task = tokio::spawn(async move {
        // staring the API events in a separate thread
        let mut event_builder = events::EventsListenerBuilder::new(client);
        event_builder.add_player("#2PP").add_players(vec!["#CVJLQOLR"])
            .build(Handler) // Building the EventListener struct
            .start() // starting the continuous polling of the clan/player/war endpoints
            .await;
    });
    task.await?;
}
```

_Note_: Each endpoint has a different cache refresh time.
Each event will be fired at the exact time of new cache data in the API.

### Features

To enable the `cos` feature (to use the Clash of Stats API), add this to your `Cargo.toml`

```toml
[dependencies]
coc-rs = { version = "0.8.1", features = ["cos"] }
```

To enable the `extra` feature (which gives you extra tools), add this to your `Cargo.toml`

```toml
[dependencies]
coc-rs = { version = "0.8.1", features = ["extra"] }
```

To enable the `tracing` feature (which provides built-in debugging/tracing tools),
add this to your `Cargo.toml`

```toml
[dependencies]
coc-rs = { version = "0.8.1", features = ["tracing"] }
```

Or for all 3

```toml
[dependencies]
coc-rs = { version = "0.8.1", features = ["all"] }
```

- Alternately with `cargo add`

```sh
cargo add coc-rs --features cos # or extra...or tracing...or all (you get it)
```

## Tests

`src/lib.rs` contains examples as tests for every endpoint.

## Contributing

Contributing is fantastic and much welcomed!
If you have an issue, feel free to open an issue and start working on it.

## Disclaimer

This content is not affiliated with, endorsed, sponsored, or specifically
approved by Supercell and Supercell is not responsible for it.
For more information see [Supercell's Fan Content Policy](https://www.supercell.com/fan-content-policy).
