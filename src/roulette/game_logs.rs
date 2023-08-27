use serde::{Deserialize, Serialize};

use crate::board::slot::Slot;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct GameLog {
    pub round_number: i32,
    pub winning_slot: Slot,
}
