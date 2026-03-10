use serde_json::Value;
use std::collections::HashMap;

pub const TYPE_NAME: &str = "Extensions";

/// JSON object with extension-specific objects.
pub type Extensions = HashMap<String, Value>;
