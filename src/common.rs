//! Crate's common utilities.
//!

use serde::{Deserialize, Deserializer};

/// Replace null value in JSON with default value while deserializing.
pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
