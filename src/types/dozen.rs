use crate::{error::Error, json::deserializable::I64Deserializable};
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use super::slot_number::SlotNumber;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Dozen {
    Zero,
    One,
    Two,
    Three,
}

impl fmt::Display for Dozen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Dozen::Zero => 0,
                Dozen::One => 1,
                Dozen::Two => 2,
                Dozen::Three => 3,
            }
        );
    }
}

impl FromStr for Dozen {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Dozen::Zero),
            "One" => Ok(Dozen::One),
            "Two" => Ok(Dozen::Two),
            "Three" => Ok(Dozen::Three),
            _ => Err(Error::FromStrError {
                message: format!("Failed to convert {} to {}", s, Self::NAME),
                string: s.to_string(),
                nested_error: None,
            }),
        }
    }
}

impl TryFrom<Value> for Dozen {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}

impl I64Deserializable for Dozen {
    const NAME: &'static str = "Dozen";

    fn from_number(n: i64) -> Result<Self, Error> {
        return match n {
            0 => Ok(Dozen::Zero),
            1 => Ok(Dozen::One),
            2 => Ok(Dozen::Two),
            3 => Ok(Dozen::Three),
            _ => Err(Error::GenericError {
                message: format!("{} is not a valid {}", n, Self::NAME),
                nested_error: None,
            }),
        };
    }
}
impl Dozen {
    pub fn value(&self) -> i32 {
        return match self {
            Dozen::Zero => 0,
            Dozen::One => 1,
            Dozen::Two => 2,
            Dozen::Three => 3,
        };
    }
}

impl TryFrom<SlotNumber> for Dozen {
    type Error = Error;

    fn try_from(n: SlotNumber) -> Result<Self, Self::Error> {
        return match n {
            -1..=0 => Ok(Dozen::Zero),
            1..=12 => Ok(Dozen::One),
            13..=24 => Ok(Dozen::Two),
            25..=36 => Ok(Dozen::Three),
            _ => Err(Error::GenericError {
                message: format!("{} is not a valid slot number", n),
                nested_error: None,
            }),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_try_from() {
        let value = json!(0);
        assert_eq!(Dozen::try_from(value).unwrap(), Dozen::Zero);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Dozen::from_str("Zero").unwrap(), Dozen::Zero);
    }

    #[test]
    fn test_from_number() {
        assert_eq!(Dozen::from_number(0).unwrap(), Dozen::Zero);
    }

    #[test]
    fn test_value() {
        assert_eq!(Dozen::Zero.value(), 0);
    }
}
