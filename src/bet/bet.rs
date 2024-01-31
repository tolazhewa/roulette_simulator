use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{roulette::roulette_type::RouletteType, types::color::Color};

use super::{bet_log::BetLog, bet_state::BetState, bet_value::BetValue};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct Bet {
    pub amount_cents: i64,
    pub bet_logs: Vec<BetLog>,
    pub bet_state: BetState,
    pub bet_value: BetValue,
    pub initial_amount_cents: i64,
    pub progression_factor: i64,
}

impl Bet {
    pub fn validate(&mut self, roulette_type: Option<&RouletteType>) {
        let roulette_type = roulette_type.unwrap_or(&RouletteType::European);
        if self.bet_state != BetState::Active
            && self.amount_cents <= 0
            && self.initial_amount_cents <= 0
            && self.progression_factor <= 0
        {
            self.bet_state = BetState::Inactive;
            return;
        }

        let bet_type_valid: bool = match &self.bet_value {
            BetValue::AdjacentNumbers(adjacent_numbers) => {
                self.validate_adjacent_numbers(&adjacent_numbers.numbers, roulette_type)
            }
            BetValue::Color(color) => *color == Color::Red || *color == Color::Black,
            BetValue::Column(_) => true,
            BetValue::Dozen(_) => true,
            BetValue::EvenOdd(_) => true,
            BetValue::Half(_) => true,
            BetValue::Number(number) => {
                let n = *number;
                let res;
                if n == -1 && roulette_type == &RouletteType::American {
                    res = true
                } else {
                    res = n >= 0 && n <= 36;
                }
                res
            }
            BetValue::Row(_) => true,
            BetValue::DoubleColumn(double_column) => {
                (double_column.columns[0].value() - double_column.columns[1].value()).abs() == 1
            }
        };

        if !bet_type_valid {
            self.bet_state = BetState::Inactive;
        }
    }

    fn validate_adjacent_numbers(&self, numbers: &Vec<i8>, roulette_type: &RouletteType) -> bool {
        let num_count = numbers.len();
        let unique_numbers = numbers.iter().unique().count();
        if unique_numbers != num_count {
            return false;
        }
        if num_count > 4 || num_count < 2 {
            return false;
        }
        for number in numbers.iter() {
            if *number == -1 && *roulette_type != RouletteType::American {
                return false;
            }
            if *number < -1 || *number > 36 {
                return false;
            }
        }

        match num_count {
            2 => {
                let smaller;
                let larger;
                if numbers[0] < numbers[1] {
                    smaller = numbers[0];
                    larger = numbers[1];
                } else {
                    smaller = numbers[1];
                    larger = numbers[0];
                }
                if smaller == 0 || smaller == -1 {
                    if *roulette_type == RouletteType::American {
                        return *numbers == vec![-1, 3] || *numbers == vec![0, 1];
                    } else if *roulette_type == RouletteType::European {
                        return larger == 1 || larger == 2 || larger == 3;
                    }
                }
                if larger - smaller == 3 {
                    // horizontal case
                    return true;
                } else if larger - smaller == 1 {
                    // vertical case
                    return (larger % 3 - smaller % 3).abs() == 1;
                }
            }
            3 => {
                if *roulette_type == RouletteType::American {
                    return *numbers == vec![-1, 2, 3] || *numbers == vec![0, 1, 2];
                } else if *roulette_type == RouletteType::European {
                    return *numbers == vec![0, 2, 3] || *numbers == vec![0, 1, 2];
                }
            }
            4 => {
                let min_num = *numbers.iter().min().unwrap();
                if min_num % 3 == 0 {
                    return false;
                }
                if *numbers == vec![min_num, min_num + 1, min_num + 3, min_num + 4] {
                    return true;
                }
            }
            _ => return false,
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{
        adjacent_numbers::AdjacentNumbers, column::Column, double_column::DoubleColumn,
    };

    fn create_test_bet(bet_value: BetValue) -> Bet {
        Bet {
            amount_cents: 1000,
            bet_logs: Vec::new(),
            bet_state: BetState::Active,
            bet_value,
            initial_amount_cents: 1000,
            progression_factor: 2,
        }
    }

    // Number
    #[test]
    fn test_validate_number_valid() {
        let mut bet = create_test_bet(BetValue::Number(1));
        bet.validate(Some(&RouletteType::European));
        assert_eq!(bet.bet_state, BetState::Active);
    }

    #[test]
    fn test_validate_number_double_zero_european() {
        let mut bet = create_test_bet(BetValue::Number(-1));
        bet.validate(Some(&RouletteType::European));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_number_double_zero_american() {
        let mut bet = create_test_bet(BetValue::Number(-1));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Active);
    }

    #[test]
    fn test_validate_number_less_than_range() {
        let mut bet = create_test_bet(BetValue::Number(-2));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_number_more_than_range() {
        let mut bet = create_test_bet(BetValue::Number(37));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_double_column_valid() {
        let mut bet = create_test_bet(BetValue::DoubleColumn(DoubleColumn {
            columns: [Column::One, Column::Two],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Active);
    }

    #[test]
    fn test_validate_double_column_invalid() {
        let mut bet = create_test_bet(BetValue::DoubleColumn(DoubleColumn {
            columns: [Column::One, Column::Three],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_adjacent_numbers_invalid_duo() {
        let mut bet = create_test_bet(BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![1, 3],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_adjacent_numbers_invalid_triple() {
        let mut bet = create_test_bet(BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![1, 2, 3],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_adjacent_numbers_invalid_quad() {
        let mut bet = create_test_bet(BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![1, 2, 3, 4],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_adjacent_numbers_valid_american_invalid_european() {
        let bet_value = BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![-1, 2, 3],
        });
        let mut american_bet = create_test_bet(bet_value.clone());
        american_bet.validate(Some(&RouletteType::American));
        assert_eq!(american_bet.bet_state, BetState::Active);

        let mut european_bet = create_test_bet(bet_value);
        european_bet.validate(Some(&RouletteType::European));
        assert_eq!(european_bet.bet_state, BetState::Inactive);
    }

    #[test]
    fn test_validate_adjacent_numbers_valid_european_invalid_american() {
        let bet_value = BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![0, 2, 3],
        });
        let mut american_bet = create_test_bet(bet_value.clone());
        american_bet.validate(Some(&RouletteType::American));
        assert_eq!(american_bet.bet_state, BetState::Inactive);

        let mut european_bet = create_test_bet(bet_value);
        european_bet.validate(Some(&RouletteType::European));
        assert_eq!(european_bet.bet_state, BetState::Active);
    }

    #[test]
    fn test_validate_adjacent_numbers_repeat_numbers() {
        let mut bet = create_test_bet(BetValue::AdjacentNumbers(AdjacentNumbers {
            numbers: vec![1, 2, 2],
        }));
        bet.validate(Some(&RouletteType::American));
        assert_eq!(bet.bet_state, BetState::Inactive);
    }
}
