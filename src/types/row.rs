use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Row::Zero),
            "One" => Ok(Row::One),
            "Two" => Ok(Row::Two),
            "Three" => Ok(Row::Three),
            _ => Err(format!("{} is not a valid row", s)),
        }
    }
}

impl TryFrom<Value> for Row {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let row = value.as_str().unwrap();
        return Ok(Row::from_str(row).unwrap());
    }
}
