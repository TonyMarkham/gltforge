use error_location::ErrorLocation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("invalid {type_name} value: {value} {location}")]
    InvalidValue {
        type_name: &'static str,
        value: u32,
        location: ErrorLocation,
    },
}

pub type Result<T> = std::result::Result<T, SchemaError>;
