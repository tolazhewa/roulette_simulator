use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::I64Deserializable};

use super::slot_number::SlotNumber;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Column {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.value());
    }
}

impl FromStr for Column {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Column::Zero),
            "One" => Ok(Column::One),
            "Two" => Ok(Column::Two),
            "Three" => Ok(Column::Three),
            "Four" => Ok(Column::Four),
            "Five" => Ok(Column::Five),
            "Six" => Ok(Column::Six),
            "Seven" => Ok(Column::Seven),
            "Eight" => Ok(Column::Eight),
            "Nine" => Ok(Column::Nine),
            "Ten" => Ok(Column::Ten),
            "Eleven" => Ok(Column::Eleven),
            "Twelve" => Ok(Column::Twelve),
            _ => Err(Error::FromStrError {
                message: format!("Failed to convert {} to {}", s, Self::NAME),
                string: s.to_string(),
                nested_error: None,
            }),
        }
    }
}

impl Column {
    pub fn value(&self) -> i32 {
        return match self {
            Column::Zero => 0,
            Column::One => 1,
            Column::Two => 2,
            Column::Three => 3,
            Column::Four => 4,
            Column::Five => 5,
            Column::Six => 6,
            Column::Seven => 7,
            Column::Eight => 8,
            Column::Nine => 9,
            Column::Ten => 10,
            Column::Eleven => 11,
            Column::Twelve => 12,
        };
    }
}

impl TryFrom<Value> for Column {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Self::try_deserialize(value);
    }
}

impl I64Deserializable for Column {
    const NAME: &'static str = "Column";

    fn from_number(n: i64) -> Result<Self, Error> {
        return match n {
            0 => Ok(Column::Zero),
            1 => Ok(Column::One),
            2 => Ok(Column::Two),
            3 => Ok(Column::Three),
            4 => Ok(Column::Four),
            5 => Ok(Column::Five),
            6 => Ok(Column::Six),
            7 => Ok(Column::Seven),
            8 => Ok(Column::Eight),
            9 => Ok(Column::Nine),
            10 => Ok(Column::Ten),
            11 => Ok(Column::Eleven),
            12 => Ok(Column::Twelve),
            _ => Err(Error::GenericError {
                message: format!("{} is not a valid {}", n, Self::NAME),
                nested_error: None,
            }),
        };
    }
}

impl TryFrom<SlotNumber> for Column {
    type Error = Error;

    fn try_from(n: SlotNumber) -> Result<Self, Self::Error> {
        return match n {
            -1..=0 => Ok(Column::Zero),
            1..=36 => Self::from_number((((n - 1) / 3) + 1) as i64),
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
        let value = json!(1);
        assert_eq!(Column::try_from(value).unwrap(), Column::One);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Column::from_str("One").unwrap(), Column::One);
    }

    #[test]
    fn test_from_number() {
        assert_eq!(Column::from_number(1).unwrap(), Column::One);
    }

    #[test]
    fn test_try_from_slot_number() {
        assert_eq!(Column::try_from(1).unwrap(), Column::One);
    }

    #[test]
    fn test_value() {
        assert_eq!(Column::One.value(), 1);
    }
}
