# coc.rs
A Clash of Clans API wrapper written in rust!

# Key feature
- Asynchronous code
- Entire coverage of [Clash of clans API](https://developer.clashofclans.com)
- Email and Password Login
- Multiple Accounts Login to handle concurrent requests

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

`$ cargo add coc-rs`

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

### How to Handler Errors
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

#### Possible Error Code

400 -> BadRequestException <br/>
403 -> AuthException <br/>
404 -> NotFoundException<br/>
429 -> RateLimitException <br/>
503 -> MaintenanceException<br/>

#### Note
`src/test.rs` contains examples for every endpoint in more detail. 

# Contributing
Contributing is fantastic and much welcomed! If you have an issue, feel free to open an issue and start working on it.

# Disclaimer
This content is not affiliated with, endorsed, sponsored, or specifically
approved by Supercell and Supercell is not responsible for it.
For more information see [Supercell's Fan Content Policy](https://www.supercell.com/fan-content-policy).
