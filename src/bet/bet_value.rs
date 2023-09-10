use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::Error,
    types::{
        adjacent_numbers::AdjacentNumbers, color::Color, column::Column,
        double_column::DoubleColumn, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum BetValue {
    AdjacentNumbers(AdjacentNumbers),
    Color(Color),
    Column(Column),
    DoubleColumn(DoubleColumn),
    Dozen(Dozen),
    EvenOdd(EvenOdd),
    Half(Half),
    Number(i8),
    Row(Row),
}
impl BetValue {
    const NAME: &'static str = "BetValue";

    pub fn get_type(&self) -> String {
        return String::from(match self {
            BetValue::AdjacentNumbers(_) => "Adjacent Numbers",
            BetValue::Color(_) => "Color",
            BetValue::Column(_) => "Column",
            BetValue::DoubleColumn(_) => "DoubleColumn",
            BetValue::Dozen(_) => "Dozen",
            BetValue::EvenOdd(_) => "EvenOdd",
            BetValue::Half(_) => "Half",
            BetValue::Number(_) => "Number",
            BetValue::Row(_) => "Row",
        });
    }
    pub fn get_value_string(&self) -> String {
        return match self {
            BetValue::AdjacentNumbers(adjacent_numbers) => {
                let mut s: String = String::new();
                let numbers_len = adjacent_numbers.numbers.len();
                s.push_str("[");
                for i in 0..numbers_len - 1 {
                    s.push_str(format!("{} ", adjacent_numbers.numbers[i]).as_str());
                }
                s.push_str(format!("{}]", adjacent_numbers.numbers[numbers_len - 1]).as_str());
                return s;
            }
            BetValue::Color(color) => color.to_string(),
            BetValue::Column(column) => column.to_string(),
            BetValue::DoubleColumn(double_column) => {
                return format!(
                    "[{} {}]",
                    double_column.columns[0].value(),
                    double_column.columns[1].value()
                );
            }
            BetValue::Dozen(dozen) => dozen.to_string(),
            BetValue::EvenOdd(even_odd) => even_odd.to_string(),
            BetValue::Half(half) => half.to_string(),
            BetValue::Number(number) => number.to_string(),
            BetValue::Row(row) => row.to_string(),
        };
    }
}

impl TryFrom<Value> for BetValue {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let val_obj = value.as_object().ok_or(Error::DeserializatonError {
            message: format!(
                "Value passed onto {}::try_from is not an object",
                Self::NAME
            ),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;
        let (bet_type, bet_info) = val_obj.iter().next().ok_or(Error::DeserializatonError {
            message: format!(
                "Value passed onto {}::try_from is an empty object",
                Self::NAME
            ),
            de_str: None,
            value: Some(value.clone()),
            nested_error: None,
        })?;
        let bet_value = match bet_type.as_str() {
            "AdjacentNumbers" => {
                BetValue::AdjacentNumbers(AdjacentNumbers::try_from(bet_info.clone())?)
            }
            "Color" => BetValue::Color(Color::try_from(bet_info.clone())?),
            "Column" => BetValue::Column(Column::try_from(bet_info.clone())?),
            "DoubleColumn" => BetValue::DoubleColumn(DoubleColumn::try_from(bet_info.clone())?),
            "Dozen" => BetValue::Dozen(Dozen::try_from(bet_info.clone())?),
            "EvenOdd" => BetValue::EvenOdd(EvenOdd::try_from(bet_info.clone())?),
            "Half" => BetValue::Half(Half::try_from(bet_info.clone())?),
            "Number" => {
                let s = bet_info.as_str().ok_or(Error::DeserializatonError {
                    message: "Value passed onto BetValue::Number is not a valid string".to_string(),
                    de_str: None,
                    value: Some(value.clone()),
                    nested_error: None,
                })?;
                if s == "00" {
                    return Ok(BetValue::Number(-1 as i8));
                }
                return s
                    .parse::<i8>()
                    .map_err(|e| Error::GenericError {
                        message: format!("Failed to parse {} as i8", s),
                        nested_error: Some(Box::new(e)),
                    })
                    .map(|n| BetValue::Number(n));
            }
            "Row" => BetValue::Row(Row::try_from(bet_info.clone())?),
            _ => {
                return Err(Error::DeserializatonError {
                    message: format!("Invalid bet type: {}", bet_type),
                    de_str: None,
                    value: Some(value.clone()),
                    nested_error: None,
                });
            }
        };
        return Ok(bet_value);
    }
}
