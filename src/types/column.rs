use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

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
    type Err = String;

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
            _ => Err(format!("{} is not a valid column", s)),
        }
    }
}

impl Column {
    pub fn from_number(n: i32) -> Self {
        return match n {
            0 => Column::Zero,
            1 => Column::One,
            2 => Column::Two,
            3 => Column::Three,
            4 => Column::Four,
            5 => Column::Five,
            6 => Column::Six,
            7 => Column::Seven,
            8 => Column::Eight,
            9 => Column::Nine,
            10 => Column::Ten,
            11 => Column::Eleven,
            12 => Column::Twelve,
            _ => panic!("{} is not a valid column", n),
        };
    }
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
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        return Ok(Column::from_str(value.as_str().unwrap()).unwrap());
    }
}
