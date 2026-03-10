use serde_json::Value;

pub const TYPE_NAME: &str = "Extras";

/// Application-specific data. May be any JSON value.
pub type Extras = Value;
