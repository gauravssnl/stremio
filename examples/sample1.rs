use stremio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("user@mail.com".to_string(), "password".to_string());
    let client = client.login().await?;
    println!("client after login: {client:?}");
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
