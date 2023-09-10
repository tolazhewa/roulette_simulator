use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::StringDeserializable};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum EvenOdd {
    Even,
    Odd,
    Zero,
}

impl fmt::Display for EvenOdd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                EvenOdd::Even => "Even",
                EvenOdd::Odd => "Odd",
                EvenOdd::Zero => "Zero",
            }
        );
    }
}

impl FromStr for EvenOdd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Even" => Ok(EvenOdd::Even),
            "Odd" => Ok(EvenOdd::Odd),
            "Zero" => Ok(EvenOdd::Zero),
            _ => Err(Error::FromStrError {
                message: format!("Failed to convert {} to {}", s, Self::NAME),
                string: s.to_string(),
                nested_error: None,
            }),
        }
    }
}

impl TryFrom<Value> for EvenOdd {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}

impl StringDeserializable for EvenOdd {
    const NAME: &'static str = "EvenOdd";
}
