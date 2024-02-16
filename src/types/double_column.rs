use crate::error::Error;

use super::column::Column;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub struct DoubleColumn {
    pub columns: [Column; 2],
}

impl TryFrom<Value> for DoubleColumn {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let columns = value.as_array().ok_or_else(|| Error::DeserializatonError {
            message: "Value is not an array".to_string(),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;

        if columns.len() != 2 {
            return Err(Error::DeserializatonError {
                message: format!(
                    "Double column does not have 2 columns, has {}",
                    columns.len()
                ),
                de_str: None,
                value: Some(value),
                nested_error: None,
            });
        }
        let column1 =
            Column::try_from(columns[0].clone()).map_err(|e| Error::DeserializatonError {
                message: "Failed to convert first column".to_string(),
                de_str: None,
                value: Some(columns[0].clone()),
                nested_error: Some(Box::new(e)),
            })?;

        let column2 =
            Column::try_from(columns[1].clone()).map_err(|e| Error::DeserializatonError {
                message: "Failed to convert second column".to_string(),
                de_str: None,
                value: Some(columns[1].clone()),
                nested_error: Some(Box::new(e)),
            })?;

        return Ok(DoubleColumn {
            columns: [column1, column2],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_try_from() {
        let value = json!([0, 1]);
        let double_column = DoubleColumn::try_from(value).unwrap();
        assert_eq!(double_column.columns[0], Column::Zero);
        assert_eq!(double_column.columns[1], Column::One);
    }

    #[test]
    fn test_try_from_invalid() {
        let value = json!(["Zero"]);
        let double_column = DoubleColumn::try_from(value);
        assert!(double_column.is_err());
    }
}
