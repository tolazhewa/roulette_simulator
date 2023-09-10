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
            let num_str = number.as_str().ok_or(Error::DeserializatonError {
                message: "Value passed onto BetValue::Number is not a valid string".to_string(),
                de_str: None,
                value: Some(value.clone()),
                nested_error: None,
            })?;
            if num_str == "00" {
                numbers_vec.push(-1 as i8);
            } else {
                numbers_vec.push(num_str.parse::<i8>().map_err(|e| Error::GenericError {
                    message: format!("Failed to parse {} as i8", num_str),
                    nested_error: Some(Box::new(e)),
                })?)
            }
        }
        return Ok(AdjacentNumbers {
            numbers: numbers_vec,
        });
    }
}

impl AdjacentNumbers {
    const NAME: &'static str = "AdjacentNumbers";
}
