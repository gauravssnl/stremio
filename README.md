# stremio


[![crates.io](https://img.shields.io/crates/v/stremio.svg?logo=rust)](https://crates.io/crates/stremio)
[![docs.rs](https://docs.rs/stremio/badge.svg)](https://docs.rs/stremio/)
[![Build](https://github.com/gauravssnl/stremio/actions/workflows/rust.yml/badge.svg)](https://github.com/gauravssnl/stremio/actions/workflows/rust.yml)
![License](https://img.shields.io/crates/l/stremio)
[![deps.rs](https://deps.rs/repo/github/gauravssnl/stremio/status.svg)](https://deps.rs/repo/github/gauravssnl/stremio)

A fast & async library for [stremio.com](https://stremio.com) APIs written in Rust. 

# Example
Examples can be found in this crate's [examples directory](https://github.com/gauravssnl/stremio/tree/master/symphonia/examples).


A sample is provided below for reference.

```rust

use stremio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "user@mail.com".to_string(),
        "password".to_string(),
    );
    let client = client.login().await?;
    println!("client login result : {}", client.is_logged_in);
    println!("client auth key : {}", client.auth_key);
    let sessions = client.get_user_sessions().await?;
    println!("user sessions: {sessions:?}");
    let user = client.get_user().await?;
    println!("user details: {user:?}");
    let logout = client.logout().await?;
    println!("logout result: {logout:?}");
    Ok(())
}
```

# Usage
To use this library, the user needs to have a Stremio account and use their credentials while calling the API. If you don't have an account, please sign up first on [stremio](https://www.stremio.com/register).


Add the `stremio` dependency in Cargo.toml dependencies section :
```
stremio = "0.1.0"
```
You can follow the sample examples after adding the lib.

# Contribution
Contributions and PRs are welcome.