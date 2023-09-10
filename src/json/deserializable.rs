use crate::error::Error;
use serde_json::Value;
use std::str::FromStr;

pub trait Deserializable: Sized + FromStr<Err = Error> {
    const NAME: &'static str;

    fn try_deserialize(value: Value) -> Result<Self, Error> {
        let str = value.as_str().ok_or(Error::DeserializatonError {
            message: format!("Value passed onto {}::try_from is not a string", Self::NAME),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;
        return Self::from_str(str).map_err(|e| Error::DeserializatonError {
            message: format!("Error deserializing {}", Self::NAME),
            de_str: Some(str.to_string()),
            value: None,
            nested_error: Some(Box::new(e)),
        });
    }
}
