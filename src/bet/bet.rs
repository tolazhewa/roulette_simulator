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
                let n = get_number_from_slot_string(number);
                if n == -1 && *roulette_type != RouletteType::American {
                    false;
                }
                let res = n >= -1 && n <= 36;
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

    fn validate_adjacent_numbers(
        &self,
        numbers: &Vec<String>,
        roulette_type: &RouletteType,
    ) -> bool {
        let num_count = numbers.len();
        if num_count > 4 || num_count < 2 {
            return false;
        }
        let mut nums: Vec<i8> = Vec::new();
        for number in numbers.iter() {
            let n = get_number_from_slot_string(number);
            if n == -1 && *roulette_type != RouletteType::American {
                return false;
            }
            if n < -1 || n > 36 {
                return false;
            }
            nums.push(n);
        }

        match num_count {
            2 => {
                let n1 = nums[0];
                let n2 = nums[1];
                if (n1 - n2).abs() != 1 && (n1 - n2).abs() != 3 {
                    return false;
                }
            }
            3 => {
                let possible_triples: Vec<Vec<i8>> =
                    vec![vec![0, 1, 2], vec![0, 2, 3], vec![-1, 0, 2], vec![-1, 2, 3]];
                if !possible_triples.contains(&nums) {
                    return false;
                }
            }
            4 => {
                let min_num = *nums.iter().min().unwrap();
                if min_num % 3 == 0 {
                    return false;
                }
                let target_value: Vec<i8> = vec![min_num, min_num + 1, min_num + 3, min_num + 4];
                if nums != target_value {
                    return false;
                }
            }
            _ => return false,
        }
        return true;
    }
}

fn get_number_from_slot_string(slot_string: &String) -> i8 {
    if slot_string == "00" {
        return -1;
    }
    let number = slot_string.parse::<i8>().unwrap();
    return number;
}
