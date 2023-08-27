use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Even" => Ok(EvenOdd::Even),
            "Odd" => Ok(EvenOdd::Odd),
            "Zero" => Ok(EvenOdd::Zero),
            _ => Err(format!("{} is not a valid even odd", s)),
        }
    }
}

impl TryFrom<Value> for EvenOdd {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let even_odd = value.as_str().unwrap();
        return Ok(EvenOdd::from_str(even_odd).unwrap());
    }
}
