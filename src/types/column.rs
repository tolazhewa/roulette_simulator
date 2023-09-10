use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::{error::Error, json::deserializable::Deserializable};

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
    pub fn from_number(n: i8) -> Result<Self, Error> {
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
        let num = value.as_i64().ok_or(Error::DeserializatonError {
            message: format!("Value passed onto {}::try_from is not a number", Self::NAME),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })? as i8;
        return Self::from_number(num).map_err(|e| Error::DeserializatonError {
            message: format!("Error deserializing {}", Self::NAME),
            de_str: None,
            value: Some(value.clone()),
            nested_error: Some(Box::new(e)),
        });
    }
}

impl Deserializable for Column {
    const NAME: &'static str = "Column";
}
