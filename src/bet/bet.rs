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
    pub fn validate_bet(&mut self, roulette_type: &RouletteType) {
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
                if *number == -1 && *roulette_type != RouletteType::American {
                    false;
                }
                let res = *number >= -1 && *number <= 36;
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
                if let Some(min) = numbers.iter().min() {
                    smaller = min;
                } else {
                    return false;
                }
                if let Some(max) = numbers.iter().max() {
                    larger = max;
                } else {
                    return false;
                }
                if *smaller == -1 {
                    let possible_doubles: Vec<Vec<i8>> = vec![vec![-1, 3], vec![-1, 0]];
                    return possible_doubles.contains(numbers);
                } else if *smaller == 0 {
                    if roulette_type == &RouletteType::American {
                        return *larger == 1;
                    } else {
                        let possible_doubles: Vec<Vec<i8>> =
                            vec![vec![0, 1], vec![0, 2], vec![0, 3]];
                        return possible_doubles.contains(numbers);
                    }
                }
                if larger - smaller == 3 {
                    return true;
                } else if larger - smaller == 1 {
                    return (larger % 3 - smaller % 3).abs() <= 1;
                } else {
                    return false;
                }
            }
            3 => {
                let possible_triples: Vec<Vec<i8>> =
                    vec![vec![0, 1, 2], vec![0, 2, 3], vec![-1, 0, 2], vec![-1, 2, 3]];
                if !possible_triples.contains(&numbers) {
                    return false;
                }
            }
            4 => {
                let min_num = if let Some(min) = numbers.iter().min() {
                    *min
                } else {
                    return false;
                };
                if min_num % 3 == 0 {
                    return false;
                }
                let target_value: Vec<i8> = vec![min_num, min_num + 1, min_num + 3, min_num + 4];
                if *numbers != target_value {
                    return false;
                }
            }
            _ => return false,
        }
        return true;
    }
}
