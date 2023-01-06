use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::{
    deserialize_null_default, GET_USER_API_URL, GET_USER_SESSIONS_API_URL, LOGIN_API_URL,
    LOGOUT_API_URL,
};

/// Login API struct.
#[derive(Serialize, Deserialize)]
pub struct LoginCredential {
    pub email: String,
    pub password: String,
    pub auth_key: Option<String>,
}

impl LoginCredential {
    pub fn new(email: String, password: String, auth_key: Option<String>) -> Self {
        LoginCredential {
            email,
            password,
            auth_key,
        }
    }
}

/// API Response struct.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    // API returns `result` value as `null' on successful login.
    #[serde(deserialize_with = "deserialize_null_default")]
    pub result: HashMap<String, Value>,
    // API returns `error` value as `null' on successful login.
    #[serde(deserialize_with = "deserialize_null_default")]
    pub error: HashMap<String, Value>,
}

/// Entrypoint for interacting with the API client.
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    pub email: String,
    pub password: String,
    pub auth_key: String,
    pub is_logged_in: bool,
    pub id: String,
}

impl Client {
    /// Create new client with given email & password to be used in API calls.
    pub fn new(email: String, password: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            email,
            password,
            auth_key: String::default(),
            is_logged_in: false,
            id: String::default(),
        }
    }

    /// Set up the client with email, password & auth key to be used in API calls.
    /// Once you set up the apiKey in the client, client.login() should not be called.
    pub fn new_with_auth_key(email: String, password: String, auth_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            email,
            password,
            auth_key,
            is_logged_in: false,
            id: String::default(),
        }
    }

    /// Get the reponse by calling `login` API.
    pub async fn get_login_response(&self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let login_data = LoginCredential::new(
            self.email.clone(),
            self.password.clone(),
            Some(self.auth_key.clone()),
        );
        let response = self
            .client
            .post(LOGIN_API_URL)
            .json(&login_data)
            .send()
            .await?;
        let login_response: ApiResponse = response.json().await?;
        Ok(login_response)
    }

    /// Get the reponse by calling `getUserSessions` API.
    pub async fn get_user_sessions(&self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        println!("self auth key: {:?}", self.auth_key);
        let user_auth = UserAuth::new(self.auth_key.clone());
        let response = self
            .client
            .post(GET_USER_SESSIONS_API_URL)
            .json(&user_auth)
            .send()
            .await?;
        let response: ApiResponse = response.json().await?;
        if !response.error.is_empty() {
            println!("Get user error: {:?}", response);
        }
        Ok(response)
    }

    /// Get the reponse by calling `getUser` API.
    pub async fn get_user(&self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        if !self.is_logged_in {
            panic!("User not logged in. Login first");
        }
        let user_auth = UserAuth::new(self.auth_key.clone());
        // let json = serde_json::to_string_pretty(&user_auth).unwrap();
        // println!("JSON: {}", json);
        let response = self
            .client
            .post(GET_USER_API_URL)
            .json(&user_auth)
            .send()
            .await?;
        let response: ApiResponse = response.json().await?;
        if !response.error.is_empty() {
            panic!("Get user error: {:?}", response);
        }
        Ok(response)
    }

    /// Login the client user for API calls.
    /// This has to done first in order to consume other APIs.
    /// This method will consume the current self and prodces a new client.
    pub async fn login(self) -> Result<Client, Box<dyn std::error::Error>> {
        let login_response = self.get_login_response().await?;
        if !login_response.error.is_empty() {
            println!("Login error: {:?}", login_response);
        }
        let mut client = self.clone();
        let login_result = login_response.result;
        if let Some(auth_key) = login_result.get("authKey") {
            client.auth_key = auth_key.as_str().unwrap().to_string();
            client.is_logged_in = true;
        }
        if let Some(user) = login_result.get("user") {
            let _id = user.get("_id").unwrap();
            client.id = _id.as_str().unwrap().to_string();
        }
        Ok(client)
    }

    /// Log out the client by calling `logout` API.
    pub async fn logout(self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        if !self.is_logged_in {
            panic!("User not logged in. Login first");
        }
        let user_auth = UserAuth::new(self.auth_key.clone());
        let response = self
            .client
            .post(LOGOUT_API_URL)
            .json(&user_auth)
            .send()
            .await?;
        let response: ApiResponse = response.json().await?;
        if !response.error.is_empty() {
            println!("Get user error: {:?}", response);
        }
        Ok(response)
    }
}

/// user info struct for API response.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub _id: String,
    pub email: String,
    #[serde(alias = "fbId")]
    pub fb_id: String,
    pub fullname: String,
    pub avatar: String,
    pub anonymous: String,
    pub gdpr_consent: HashMap<String, Value>,
    pub taste: String,
    pub lang: String,
    #[serde(alias = "dateRegistered")]
    pub date_registered: String,
    #[serde(alias = "lastModified")]
    pub last_modified: String,
    pub stremio_addons: String,
    pub premium_expire: String,
}

/// UserAuth struct for API calls.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    auth_key: String,
}

impl UserAuth {
    fn new(auth_key: String) -> Self {
        Self { auth_key }
    }
}
