use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::I64Deserializable};

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

impl I64Deserializable for Row {
    const NAME: &'static str = "Row";

    fn from_number(n: i64) -> Result<Self, Error> {
        return match n {
            0 => Ok(Row::Zero),
            1 => Ok(Row::One),
            2 => Ok(Row::Two),
            3 => Ok(Row::Three),
            _ => Err(Error::GenericError {
                message: format!("{} is not a valid {}", n, Self::NAME),
                nested_error: None,
            }),
        };
    }
}

impl TryFrom<Value> for Row {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}
