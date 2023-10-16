use serde::{Deserialize, Deserializer};
use std::{fmt, result, str};

use crate::Num;

/// Used to check if the value is a number or string.
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumeric {
    String(String),
    Numeric(Num),
}

/// Deserializes from a string, using the default value if there is an error or is null.
pub(crate) fn from_str<'de, S, D>(deserializer: D) -> result::Result<S, D::Error>
where
    S: str::FromStr + Default,
    S::Err: fmt::Display,
    D: Deserializer<'de>,
{
    // Catches strings, null values, floats / doubles, and integers.
    // Null values default to 0.
    let s: String = match Deserialize::deserialize(deserializer) {
        Ok(value) => match value {
            StringOrNumeric::String(value) => value,
            StringOrNumeric::Numeric(value) => value.to_string(),
        },
        Err(_) => String::default(),
    };

    Ok(S::from_str(&s).unwrap_or_default())
}
