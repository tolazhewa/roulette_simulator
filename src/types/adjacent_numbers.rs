use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AdjacentNumbers {
    pub numbers: Vec<i8>,
}

impl TryFrom<Value> for AdjacentNumbers {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let numbers = value.as_array().ok_or(Error::DeserializatonError {
            message: format!("Value passed onto {}::try_from is not an array", Self::NAME),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;
        let mut numbers_vec: Vec<i8> = Vec::new();
        for number in numbers {
            numbers_vec.push(number.as_i64().ok_or(Error::DeserializatonError {
                message: format!(
                    "Value passed onto {}::try_from is not an array of integers",
                    Self::NAME
                ),
                de_str: None,
                value: Some(value.clone()),
                nested_error: None,
            })? as i8);
        }
        return Ok(AdjacentNumbers {
            numbers: numbers_vec,
        });
    }
}

impl AdjacentNumbers {
    const NAME: &'static str = "AdjacentNumbers";
}
