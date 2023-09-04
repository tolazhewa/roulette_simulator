use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    adjacent_numbers::AdjacentNumbers, color::Color, column::Column, double_column::DoubleColumn,
    dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
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
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let (bet_type, bet_info) = value.as_object().unwrap().iter().next().unwrap();
        return Ok(match bet_type.as_str() {
            "AdjacentNumbers" => {
                BetValue::AdjacentNumbers(AdjacentNumbers::try_from(bet_info.clone()).unwrap())
            }
            "Color" => BetValue::Color(Color::try_from(bet_info.clone()).unwrap()),
            "Column" => BetValue::Column(Column::try_from(bet_info.clone()).unwrap()),
            "DoubleColumn" => {
                BetValue::DoubleColumn(DoubleColumn::try_from(bet_info.clone()).unwrap())
            }
            "Dozen" => BetValue::Dozen(Dozen::try_from(bet_info.clone()).unwrap()),
            "EvenOdd" => BetValue::EvenOdd(EvenOdd::try_from(bet_info.clone()).unwrap()),
            "Half" => BetValue::Half(Half::try_from(bet_info.clone()).unwrap()),
            "Number" => BetValue::Number(bet_info.as_i64().unwrap() as i8),
            "Row" => BetValue::Row(Row::try_from(bet_info.clone()).unwrap()),
            _ => return Err(()),
        });
    }
}
