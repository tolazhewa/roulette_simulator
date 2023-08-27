use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AdjacentNumbers {
    pub numbers: Vec<String>,
}

impl TryFrom<Value> for AdjacentNumbers {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let numbers = value.as_array().unwrap();
        return Ok(AdjacentNumbers {
            numbers: numbers
                .iter()
                .map(|num| num.as_str().unwrap().to_string())
                .collect(),
        });
    }
}
