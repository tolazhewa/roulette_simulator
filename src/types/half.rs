use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::I64Deserializable};

use super::from_slot_number::FromSlotNumber;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Half {
    Zero,
    One,
    Two,
}

impl fmt::Display for Half {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Half::Zero => 0,
                Half::One => 1,
                Half::Two => 2,
            }
        );
    }
}

impl FromStr for Half {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Half::Zero),
            "One" => Ok(Half::One),
            "Two" => Ok(Half::Two),
            _ => Err(Error::FromStrError {
                message: format!("Failed to convert {} to {}", s, Self::NAME),
                string: s.to_string(),
                nested_error: None,
            }),
        }
    }
}

impl TryFrom<Value> for Half {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}

impl I64Deserializable for Half {
    const NAME: &'static str = "Half";

    fn from_number(n: i64) -> Result<Self, Error> {
        return match n {
            0 => Ok(Half::Zero),
            1 => Ok(Half::One),
            2 => Ok(Half::Two),
            _ => Err(Error::GenericError {
                message: format!("Failed to convert {} to {}", n, Self::NAME),
                nested_error: None,
            }),
        };
    }
}

impl Half {
    pub fn value(&self) -> i32 {
        return match self {
            Half::Zero => 0,
            Half::One => 1,
            Half::Two => 2,
        };
    }
}

impl FromSlotNumber for Half {
    type Output = Half;

    fn from_slot_number(n: i64) -> Result<Self::Output, Error> {
        return match n {
            -1..=0 => Ok(Half::Zero),
            1..=18 => Ok(Half::One),
            19..=36 => Ok(Half::Two),
            _ => Err(Error::GenericError {
                message: format!("{} is not a valid slot number", n),
                nested_error: None,
            }),
        };
    }
}
