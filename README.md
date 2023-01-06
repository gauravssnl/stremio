# stremio

A fast & async library for Stremio, written in Rust. 

# Example
Examples can be found in [this crate's examples directory].


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
To use this library, the user needs to have Stremio account and use their credentails while calling the API. If you don't have an account, please sign up first.

Add the `stremio` dependency in Cargo.toml dependencies section :
```
stremio = "0.1.0"
```
You can follow the sample examples after adding the lib.

# Contribution
Contributions and PRs are welcome.