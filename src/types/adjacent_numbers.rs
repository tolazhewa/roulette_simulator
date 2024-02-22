use serde_json::Value;

use crate::error::Error;

use super::slot_number::SlotNumber;

#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct AdjacentNumbers {
    pub numbers: Vec<SlotNumber>,
}

impl TryFrom<Value> for AdjacentNumbers {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let numbers = value.as_array().ok_or(Error::DeserializatonError {
            message: format!("Value passed onto {}::try_from is not an array", Self::NAME),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;
        let mut numbers_vec: Vec<SlotNumber> = Vec::new();
        for number in numbers {
            let num_str = number.as_str().ok_or(Error::DeserializatonError {
                message: "Value passed onto BetValue::Number is not a valid string".to_string(),
                de_str: None,
                value: Some(value.clone()),
                nested_error: None,
            })?;
            if num_str == "00" {
                numbers_vec.push(-1 as SlotNumber);
            } else {
                numbers_vec.push(num_str.parse::<SlotNumber>().map_err(|e| {
                    Error::GenericError {
                        message: format!("Failed to parse {} as SlotNumber", num_str),
                        nested_error: Some(Box::new(e)),
                    }
                })?)
            }
        }
        return Ok(AdjacentNumbers {
            numbers: numbers_vec,
        });
    }
}

impl AdjacentNumbers {
    const NAME: &'static str = "AdjacentNumbers";
}

// create unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_try_from() {
        let value = json!(["1", "2"]);
        let adjacent_numbers = AdjacentNumbers::try_from(value).unwrap();
        assert_eq!(
            adjacent_numbers,
            AdjacentNumbers {
                numbers: vec![1, 2]
            }
        );
    }

    #[test]
    fn test_try_from_00() {
        let value = json!(["00", "1"]);
        let adjacent_numbers = AdjacentNumbers::try_from(value).unwrap();
        assert_eq!(
            adjacent_numbers,
            AdjacentNumbers {
                numbers: vec![-1, 1]
            }
        );
    }
}
