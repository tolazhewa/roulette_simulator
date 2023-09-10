use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::error::Error;
use crate::json::deserializable::Deserializable;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Row {
    Zero,
    One,
    Two,
    Three,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Row::Zero => 0,
                Row::One => 1,
                Row::Two => 2,
                Row::Three => 3,
            }
        );
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Row::Zero),
            "One" => Ok(Row::One),
            "Two" => Ok(Row::Two),
            "Three" => Ok(Row::Three),
            _ => Err(Error::FromStrError {
                message: format!("Failed to convert {} to {}", s, Self::NAME),
                string: s.to_string(),
                nested_error: None,
            }),
        }
    }
}

impl Row {
    pub fn value(&self) -> i32 {
        return match self {
            Row::Zero => 0,
            Row::One => 1,
            Row::Two => 2,
            Row::Three => 3,
        };
    }
}

impl TryFrom<Value> for Row {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}

impl Deserializable for Row {
    const NAME: &'static str = "Row";
}
