//! Types used for API Requests & Responses.
//!

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::deserialize_null_default;

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

/// Generic API Response struct.
/// Used when result type & error type is not known.
/// For example, when result type from API is an array
/// then `GenericApiResponse` can be used for deserializing.
#[derive(Serialize, Deserialize, Debug)]
pub struct GenericApiResponse {
    pub result: Value,
    pub error: Value,
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

/// UserAuth struct for API calls.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    auth_key: String,
}

impl UserAuth {
    pub fn new(auth_key: String) -> Self {
        Self { auth_key }
    }
}

/// GetAddonCollection struct for `addonCollectionGet` API.
#[derive(Debug, Serialize, Default)]
pub struct GetAddonCollection {
    #[serde(rename = "addFromURL")]
    add_from_url: Vec<String>,
    #[serde(rename = "authKey")]
    auth_key: String,
    update: bool,
}

impl GetAddonCollection {
    pub fn new(auth_key: String) -> Self {
        Self {
            auth_key,
            ..Default::default()
        }
    }
}

/// DatastoreMeta struct for `datastoreMeta` API.
#[derive(Debug, Serialize)]
pub struct DatastoreMeta {
    #[serde(rename = "authKey")]
    auth_key: String,
    collection: String,
    from: String,
}

impl DatastoreMeta {
    pub fn new_with_auth_key(auth_key: String) -> Self {
        Self {
            auth_key,
            ..Default::default()
        }
    }
}

impl Default for DatastoreMeta {
    fn default() -> Self {
        Self {
            auth_key: String::default(),
            collection: "libraryItem".to_string(),
            from: "linvo-p2p-sync".to_string(),
        }
    }
}
