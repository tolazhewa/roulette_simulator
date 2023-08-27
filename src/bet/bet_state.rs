use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum BetState {
    Won,
    Lost,
    Active,
    Inactive,
}

impl FromStr for BetState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Won" => Ok(BetState::Won),
            "Lost" => Ok(BetState::Lost),
            "Active" => Ok(BetState::Active),
            "Inactive" => Ok(BetState::Inactive),
            _ => Err(format!("{} is not a valid bet type", s)),
        }
    }
}

impl fmt::Display for BetState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                BetState::Won => "Won",
                BetState::Lost => "Lost",
                BetState::Inactive => "Inactive",
                BetState::Active => "Active",
            }
        );
    }
}
