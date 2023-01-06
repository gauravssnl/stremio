use serde::{Deserialize, Deserializer};

pub(crate) const LOGIN_API_URL: &str = "https://api.strem.io/api/login";
pub(crate) const GET_USER_API_URL: &str = "https://api.strem.io/api/getUser";
pub(crate) const GET_USER_SESSIONS_API_URL: &str = "https://api.strem.io/api/getUserSessions";
pub(crate) const LOGOUT_API_URL: &str = "https://api.strem.io/api/logout";

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
