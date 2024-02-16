use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::StringDeserializable};

use super::slot_number::SlotNumber;

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

impl TryFrom<SlotNumber> for EvenOdd {
    type Error = Error;

    fn try_from(n: SlotNumber) -> Result<Self, Self::Error> {
        return match n {
            -1..=0 => Ok(EvenOdd::Zero),
            1..=36 => {
                if n % 2 == 0 {
                    Ok(EvenOdd::Even)
                } else {
                    Ok(EvenOdd::Odd)
                }
            }
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
        let value = json!("Even");
        assert_eq!(EvenOdd::try_from(value).unwrap(), EvenOdd::Even);
    }

    #[test]
    fn test_try_from_zero() {
        let value = json!("Zero");
        assert_eq!(EvenOdd::try_from(value).unwrap(), EvenOdd::Zero);
    }

    #[test]
    fn test_try_from_odd() {
        let value = json!("Odd");
        assert_eq!(EvenOdd::try_from(value).unwrap(), EvenOdd::Odd);
    }

    #[test]
    fn test_try_from_invalid() {
        let value = json!("Invalid");
        assert!(EvenOdd::try_from(value).is_err());
    }

    #[test]
    fn test_try_from_slot_number_zero() {
        assert_eq!(EvenOdd::try_from(0).unwrap(), EvenOdd::Zero);
    }

    #[test]
    fn test_try_from_slot_number_even() {
        assert_eq!(EvenOdd::try_from(2).unwrap(), EvenOdd::Even);
    }

    #[test]
    fn test_try_from_slot_number_odd() {
        assert_eq!(EvenOdd::try_from(3).unwrap(), EvenOdd::Odd);
    }

    #[test]
    fn test_try_from_slot_number_invalid() {
        assert_eq!(
            EvenOdd::try_from(37).unwrap_err().to_string(),
            Error::GenericError {
                message: "37 is not a valid slot number".to_string(),
                nested_error: None,
            }
            .to_string()
        );
    }
}
