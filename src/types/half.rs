use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Half::Zero),
            "One" => Ok(Half::One),
            "Two" => Ok(Half::Two),
            _ => Err(format!("{} is not a valid half", s)),
        }
    }
}

impl TryFrom<Value> for Half {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let half = value.as_str().unwrap();
        return Ok(Half::from_str(half).unwrap());
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
