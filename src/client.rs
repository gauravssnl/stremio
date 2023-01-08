use std::collections::HashMap;

use anyhow::{anyhow, Result};

use reqwest;
use serde_json::Value;

use crate::common::{
    ApiResponse, DatastoreMeta, GenericApiResponse, GetAddonCollection, LoginCredential, UserAuth,
    DATASTORE_META_API_URL, GET_ADDON_COLLECTION_API_URL, GET_USER_API_URL,
    GET_USER_SESSIONS_API_URL, LOGIN_API_URL, LOGOUT_API_URL,
};

/// Entrypoint for interacting with the API client.
#[derive(Debug, Clone, Default)]
pub struct Client {
    client: reqwest::Client,
    pub email: String,
    pub password: String,
    pub auth_key: String,
    pub is_logged_in: bool,
    pub user_info: HashMap<String, Value>,
}

impl Client {
    /// Create new client with given email & password to be used in API calls.
    pub fn new(email: String, password: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            email,
            password,
            ..Default::default()
        }
    }

    /// Set up the client with email, password & auth key to be used in API calls.
    /// Once you set up the apiKey in the client, client.login() should not be called.
    /// It is assumed that the user has logged in & got the API token from somewhere.
    pub fn new_with_auth_key(email: String, password: String, auth_key: String) -> Self {
        let is_logged_in = !auth_key.is_empty();
        Self {
            client: reqwest::Client::new(),
            email,
            password,
            auth_key,
            is_logged_in,
            ..Default::default()
        }
    }

    /// Get the reponse by calling `login` API.
    pub async fn get_login_response(&self) -> Result<ApiResponse> {
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
    pub async fn get_user_sessions(&self) -> Result<ApiResponse> {
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
    pub async fn get_user(&self) -> Result<ApiResponse> {
        if !self.is_logged_in {
            return Err(anyhow!("User not logged in. Login first"));
        }
        let user_auth = UserAuth::new(self.auth_key.clone());
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
    pub async fn login(self) -> Result<Client> {
        let login_response = self.get_login_response().await?;
        if self.is_logged_in {
            return Err(anyhow!("User already logged in."));
        }
        let mut client = self.clone();
        let login_result = &login_response.result;
        if let Some(auth_key) = login_result.get("authKey") {
            client.auth_key = auth_key.as_str().unwrap().to_string();
            client.is_logged_in = true;
            client.user_info = login_result.clone();
        } else {
            return Err(anyhow!(
                "User not logged in. Login API response: {:?}",
                login_response
            ));
        }
        Ok(client)
    }

    /// Log out the client by calling `logout` API.
    pub async fn logout(self) -> Result<ApiResponse> {
        if !self.is_logged_in {
            return Err(anyhow!("User not logged in. Login first"));
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
            return Err(anyhow!("Get user error: {:?}", response));
        }
        Ok(response)
    }

    /// Get Addon collection.
    pub async fn get_addon_collection(&self) -> Result<ApiResponse> {
        if !self.is_logged_in {
            return Err(anyhow!("User not logged in. Login first"));
        }
        let data = GetAddonCollection::new(self.auth_key.clone());
        let response = self
            .client
            .post(GET_ADDON_COLLECTION_API_URL)
            .json(&data)
            .send()
            .await?;
        let response: ApiResponse = response.json().await?;
        if !response.error.is_empty() {
            return Err(anyhow!("Get addon collection error: {:?}", response));
        }
        Ok(response)
    }

    /// Get Datastore meta.
    pub async fn get_datastore_meta(&self) -> Result<GenericApiResponse> {
        if !self.is_logged_in {
            return Err(anyhow!("User not logged in. Login first"));
        }
        let data = DatastoreMeta::new_with_auth_key(self.auth_key.clone());
        let response = self
            .client
            .post(DATASTORE_META_API_URL)
            .json(&data)
            .send()
            .await?;
        let response: GenericApiResponse = response.json().await?;
        if !response.error.is_empty() {
            return Err(anyhow!("Get Datastore Meta error: {:?}", response));
        }
        Ok(response)
    }
}
