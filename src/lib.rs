//! # stremio
//!
//! Stremio is a fast & async library for Stremio, written in Rust.
//! This library provides features to call Stremio APIs.
//!
//! ## Examples
//! Examples can be found in [this crate's examples directory].
//!
//! A sample is provided here.
//!
//! ```rust, no_run
//! use stremio::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(
//!       "user@email.com".to_string(),
//!        "password".to_string());
//!     let client = client.login().await?;
//!     println!("client login result : {}", client.is_logged_in);
//!     println!("client auth key : {}", client.auth_key);
//!     let sessions = client.get_user_sessions().await?;
//!     println!("user sessions: {sessions:?}");
//!     let user = client.get_user().await?;
//!     println!("user details: {user:?}");
//!     let logout = client.logout().await?;
//!     println!("logout result: {logout:?}");
//! Ok(())
//!
//! }
//! ```

pub mod client;
mod common;

pub use client::ApiResponse;
pub use client::Client;
