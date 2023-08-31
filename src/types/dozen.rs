use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Dozen::Zero),
            "One" => Ok(Dozen::One),
            "Two" => Ok(Dozen::Two),
            "Three" => Ok(Dozen::Three),
            _ => Err(format!("{} is not a valid dozen", s)),
        }
    }
}

impl TryFrom<Value> for Dozen {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let dozen = value.as_str().unwrap();
        return Ok(Dozen::from_str(dozen).unwrap());
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
