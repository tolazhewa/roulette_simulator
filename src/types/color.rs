use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Color {
    Green,
    Black,
    Red,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Color::Green => "Green",
                Color::Black => "Black",
                Color::Red => "Red",
            }
        );
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Green" => Ok(Color::Green),
            "Black" => Ok(Color::Black),
            "Red" => Ok(Color::Red),
            _ => Err(format!("{} is not a valid color", s)),
        }
    }
}

impl TryFrom<Value> for Color {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let color = value.as_str().unwrap();
        return Ok(Color::from_str(color).unwrap());
    }
}
