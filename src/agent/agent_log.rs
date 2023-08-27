use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct AgentLog {
    pub round_number: i32,
    pub balance_cents: i64,
}
