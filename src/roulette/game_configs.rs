use serde::{Deserialize, Serialize};

use super::roulette_type::RouletteType;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub number_of_rounds: i32,
    pub number_of_games: i32,
    pub allow_negative_balance: bool,
    pub roulette_type: Option<RouletteType>,
}
