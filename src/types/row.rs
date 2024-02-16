use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::I64Deserializable};

use super::slot_number::SlotNumber;

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

impl TryFrom<SlotNumber> for Row {
    type Error = Error;

    fn try_from(n: SlotNumber) -> Result<Self, Self::Error> {
        return match n {
            -1..=0 => Ok(Row::Zero),
            1..=36 => Self::from_number((((n - 1) % 3) + 1).into()),
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

    #[test]
    fn test_from_str() {
        assert_eq!(Row::from_str("Zero").unwrap(), Row::Zero);
        assert_eq!(Row::from_str("One").unwrap(), Row::One);
        assert_eq!(Row::from_str("Two").unwrap(), Row::Two);
        assert_eq!(Row::from_str("Three").unwrap(), Row::Three);
    }

    #[test]
    fn test_value() {
        assert_eq!(Row::Zero.value(), 0);
        assert_eq!(Row::One.value(), 1);
        assert_eq!(Row::Two.value(), 2);
        assert_eq!(Row::Three.value(), 3);
    }

    #[test]
    fn test_from_number() {
        assert_eq!(Row::from_number(0).unwrap(), Row::Zero);
        assert_eq!(Row::from_number(1).unwrap(), Row::One);
        assert_eq!(Row::from_number(2).unwrap(), Row::Two);
        assert_eq!(Row::from_number(3).unwrap(), Row::Three);
    }

    #[test]
    fn test_try_from_slot_number() {
        assert_eq!(Row::try_from(0).unwrap(), Row::Zero);
        assert_eq!(Row::try_from(1).unwrap(), Row::One);
        assert_eq!(Row::try_from(2).unwrap(), Row::Two);
        assert_eq!(Row::try_from(3).unwrap(), Row::Three);
    }
}