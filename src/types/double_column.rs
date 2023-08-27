use super::column::Column;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub struct DoubleColumn {
    pub columns: [Column; 2],
}

impl TryFrom<Value> for DoubleColumn {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let columns = value.as_array().unwrap();
        return Ok(DoubleColumn {
            columns: [
                Column::try_from(columns[0].clone()).unwrap(),
                Column::try_from(columns[1].clone()).unwrap(),
            ],
        });
    }
}
