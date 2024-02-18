use serde::{de::Error, Deserialize};

use crate::utils;

/// Deserializes a [`String`] that must not be empty or whitespace.
pub(crate) fn de_nonempty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if utils::is_empty_or_whitespace(&s) {
        return Err(D::Error::custom(
            "value must be non-empty and cannot contain only whitespace",
        ));
    }

    Ok(s)
}

// used because serde default requires a function,
// see: https://github.com/serde-rs/serde/issues/368
pub(crate) const fn default_true() -> bool {
    true
}

// used because serde skip_serializing_if requires a function
pub(crate) const fn is_true(b: &bool) -> bool {
    *b
}

// used because serde skip_serializing_if requires a function
pub(crate) const fn is_false(b: &bool) -> bool {
    !*b
}
