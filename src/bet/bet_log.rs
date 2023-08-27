use serde::{Deserialize, Serialize};

use super::bet_state::BetState;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct BetLog {
    pub round_number: i32,
    pub amount_cents: i64,
    pub bet_state: BetState,
}
