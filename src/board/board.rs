use super::slot::Slot;
use crate::{
    error::Error,
    roulette::roulette_type::RouletteType,
    types::{
        color::Color, column::Column, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
        slot_number::SlotNumber,
    },
};
use core::fmt;
use itertools::Itertools;
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Board {
    pub slots: Vec<Slot>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut even_odd_to_count: HashMap<EvenOdd, usize> = HashMap::new();
        let mut color_to_count: HashMap<Color, usize> = HashMap::new();
        let mut dozen_to_count: HashMap<Dozen, usize> = HashMap::new();
        let mut half_to_count: HashMap<Half, usize> = HashMap::new();
        let mut row_to_count: HashMap<Row, usize> = HashMap::new();
        let mut column_to_count: HashMap<Column, usize> = HashMap::new();
        let mut s = String::new();

        for slot in self.slots.iter() {
            even_odd_to_count
                .entry(slot.even_odd)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            color_to_count
                .entry(slot.color)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            dozen_to_count
                .entry(slot.dozen)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            half_to_count
                .entry(slot.half)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            row_to_count
                .entry(slot.row)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            column_to_count
                .entry(slot.column)
                .and_modify(|size| *size += 1)
                .or_insert(1);
        }
        let num_of_slot_numbers = self.slots.len();
        let mut numbers: HashSet<SlotNumber> = HashSet::new();
        let mut has_dup = false;
        for slot in self.slots.iter() {
            if numbers.contains(&slot.number) {
                has_dup = true;
                break;
            } else {
                numbers.insert(slot.number.clone());
            }
        }
        s.push_str("\nBOARD PROPERTIES\n\n");
        s.push_str("Evens-Odd\n");
        s.push_str("------------------------\n");
        s.push_str(&format!(
            "Number of {}: {}\n",
            EvenOdd::Zero,
            even_odd_to_count[&EvenOdd::Zero]
        ));
        s.push_str(&format!(
            "Number of {}: {}\n",
            EvenOdd::Even,
            even_odd_to_count[&EvenOdd::Even]
        ));
        s.push_str(&format!(
            "Number of {}: {}\n",
            EvenOdd::Odd,
            even_odd_to_count[&EvenOdd::Odd]
        ));

        s.push_str("\nColor\n");
        s.push_str("------------------------\n");
        s.push_str(&format!(
            "Number of {}: {}\n",
            Color::Green,
            color_to_count[&Color::Green]
        ));
        s.push_str(&format!(
            "Number of {}: {}\n",
            Color::Red,
            color_to_count[&Color::Red]
        ));
        s.push_str(&format!(
            "Number of {}: {}\n",
            Color::Black,
            color_to_count[&Color::Black]
        ));
        s.push_str("\nDozen\n");
        s.push_str("------------------------\n");
        dozen_to_count
            .iter()
            .map(|(dozen, count)| (*dozen, *count))
            .collect::<Vec<(Dozen, usize)>>()
            .into_iter()
            .sorted_by(|(dozen_a, _), (dozen_b, _)| dozen_a.value().cmp(&dozen_b.value()))
            .for_each(|(dozen, count)| {
                s.push_str(&format!("Number of {}: {}\n", dozen, count));
            });
        s.push_str("\nHalf\n");
        s.push_str("------------------------\n");
        half_to_count
            .iter()
            .map(|(half, count)| (*half, *count))
            .collect::<Vec<(Half, usize)>>()
            .into_iter()
            .sorted_by(|(half_a, _), (half_b, _)| half_a.value().cmp(&half_b.value()))
            .for_each(|(half, count)| {
                s.push_str(&format!("Number of {}: {}\n", half, count));
            });
        s.push_str("\nRow\n");
        s.push_str("------------------------\n");
        row_to_count
            .iter()
            .map(|(row, count)| (*row, *count))
            .collect::<Vec<(Row, usize)>>()
            .into_iter()
            .sorted_by(|(row_a, _), (row_b, _)| row_a.value().cmp(&row_b.value()))
            .for_each(|(row, count)| {
                s.push_str(&format!("Number of {}: {}\n", row, count));
            });
        s.push_str("\nColumn\n");
        s.push_str("------------------------\n");
        column_to_count
            .iter()
            .map(|(column, count)| (*column, *count))
            .collect::<Vec<(Column, usize)>>()
            .into_iter()
            .sorted_by(|(column_a, _), (column_b, _)| column_a.value().cmp(&column_b.value()))
            .for_each(|(column, count)| {
                s.push_str(&format!("Number of {}: {}\n", column, count));
            });
        s.push_str("------------------------\n");
        s.push_str("\nNumber\n");
        s.push_str(&format!("Number of slot number: {}\n", num_of_slot_numbers));
        s.push_str(&format!("All numbers are unique? {}\n", !has_dup));
        s.push_str(&format!("------------------------\n\n"));
        return write!(f, "{}", s);
    }
}

impl Board {
    pub fn generate(roulette_type: &RouletteType) -> Result<Self, Error> {
        let mut rng = rand::thread_rng();
        let mut slots: Vec<Slot> = Vec::new();
        slots.push(Slot {
            color: Color::Green,
            number: 0,
            even_odd: EvenOdd::Zero,
            dozen: Dozen::Zero,
            half: Half::Zero,
            row: Row::Zero,
            column: Column::Zero,
        });
        if *roulette_type == RouletteType::American {
            slots.push(Slot {
                color: Color::Green,
                number: -1,
                even_odd: EvenOdd::Zero,
                dozen: Dozen::Zero,
                half: Half::Zero,
                row: Row::Zero,
                column: Column::Zero,
            });
        }
        let mut color_counter: HashMap<Color, i32> = HashMap::new();
        color_counter.insert(Color::Red, 18);
        color_counter.insert(Color::Black, 18);
        for n in 1..=36 {
            let color: Color = get_color(&color_counter, &mut rng)?;
            color_counter.entry(color).and_modify(|count| *count -= 1);

            let even_odd: EvenOdd = EvenOdd::try_from(n)?;
            let number: SlotNumber = n as SlotNumber;
            let dozen: Dozen = Dozen::try_from(n)?;
            let half: Half = Half::try_from(n)?;
            let row: Row = Row::try_from(n)?;
            let column: Column = Column::try_from(n)?;
            slots.push(Slot {
                color,
                number,
                even_odd,
                dozen,
                half,
                row,
                column,
            });
        }
        return Ok(Board { slots });
    }
}

fn get_color(color_counter: &HashMap<Color, i32>, rng: &mut ThreadRng) -> Result<Color, Error> {
    let color: Color;
    if color_counter.contains_key(&Color::Black) && color_counter.contains_key(&Color::Red) {
        if color_counter[&Color::Red] <= 0 && color_counter[&Color::Black] <= 0 {
            return Err(Error::GenericError {
                message: format!(
                    "There are no more Red or Black slots produce: {:?}",
                    color_counter
                ),
                nested_error: None,
            });
        } else if color_counter[&Color::Black] <= 0 {
            return Ok(Color::Red);
        } else if color_counter[&Color::Red] <= 0 {
            return Ok(Color::Black);
        }
    }
    color = if rng.gen::<f64>() > 0.5 {
        Color::Red
    } else {
        Color::Black
    };
    return Ok(color);
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::error::Error;
    use crate::types::color::Color;
    use crate::types::column::Column;
    use crate::types::dozen::Dozen;
    use crate::types::half::Half;
    use crate::types::row::Row;
    use crate::types::slot_number::SlotNumber;
    use crate::{roulette::roulette_type::RouletteType, types::even_odd::EvenOdd};

    use super::{get_color, Board};

    #[test]
    fn test_get_color_only_red() {
        let mut rng = rand::thread_rng();
        let mut color_count_map = HashMap::new();
        color_count_map.insert(Color::Red, 14);
        color_count_map.insert(Color::Black, 0);
        let color = get_color(&color_count_map, &mut rng);
        assert!(color.is_ok_and(|c| c == Color::Red));
    }

    #[test]
    fn test_get_color_only_black() {
        let mut rng = rand::thread_rng();
        let mut color_count_map = HashMap::new();
        color_count_map.insert(Color::Red, 0);
        color_count_map.insert(Color::Black, 6);
        let result = get_color(&color_count_map, &mut rng);
        assert!(result.is_ok_and(|c| c == Color::Black));
    }

    #[test]
    fn test_get_color_neither_error() {
        let mut rng = rand::thread_rng();
        let mut color_count_map = HashMap::new();
        color_count_map.insert(Color::Red, 0);
        color_count_map.insert(Color::Black, 0);
        let result = get_color(&color_count_map, &mut rng);
        assert!(matches!(
            result,
            Err(Error::GenericError {
                message: _,
                nested_error: _
            })
        ));
    }
    #[test]
    fn test_get_color_both_counters_have_capacity() {
        let mut rng = rand::thread_rng();
        let mut color_count_map = HashMap::new();
        color_count_map.insert(Color::Red, 4);
        color_count_map.insert(Color::Black, 6);
        let result = get_color(&color_count_map, &mut rng);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_color_no_counter() {
        let mut rng = rand::thread_rng();
        let result = get_color(&HashMap::new(), &mut rng);
        assert!(result.is_ok());
    }

    #[test]
    fn test_board_generation_european() {
        let result = Board::generate(&RouletteType::European);
        assert!(result.is_ok());
        let board = result.unwrap();
        let mut even_odd_to_count: HashMap<EvenOdd, usize> = HashMap::new();
        let mut color_to_count: HashMap<Color, usize> = HashMap::new();
        let mut dozen_to_count: HashMap<Dozen, usize> = HashMap::new();
        let mut half_to_count: HashMap<Half, usize> = HashMap::new();
        let mut row_to_count: HashMap<Row, usize> = HashMap::new();
        let mut column_to_count: HashMap<Column, usize> = HashMap::new();

        for slot in board.slots.iter() {
            even_odd_to_count
                .entry(slot.even_odd)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            color_to_count
                .entry(slot.color)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            dozen_to_count
                .entry(slot.dozen)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            half_to_count
                .entry(slot.half)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            row_to_count
                .entry(slot.row)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            column_to_count
                .entry(slot.column)
                .and_modify(|size| *size += 1)
                .or_insert(1);
        }
        let num_of_slot_numbers = board.slots.len();
        let mut numbers: HashSet<SlotNumber> = HashSet::new();
        let mut has_dup = false;
        for slot in board.slots.iter() {
            assert!(slot.number >= 0 && slot.number <= 36);
            if numbers.contains(&slot.number) {
                has_dup = true;
                break;
            } else {
                numbers.insert(slot.number.clone());
            }
        }

        assert!(!has_dup);
        assert_eq!(num_of_slot_numbers, 37);
        assert_eq!(even_odd_to_count[&EvenOdd::Even], 18);
        assert_eq!(even_odd_to_count[&EvenOdd::Odd], 18);
        assert_eq!(even_odd_to_count[&EvenOdd::Zero], 1);
        assert_eq!(color_to_count[&Color::Red], 18);
        assert_eq!(color_to_count[&Color::Black], 18);
        assert_eq!(color_to_count[&Color::Green], 1);
        assert_eq!(dozen_to_count[&Dozen::One], 12);
        assert_eq!(dozen_to_count[&Dozen::Two], 12);
        assert_eq!(dozen_to_count[&Dozen::Three], 12);
        assert_eq!(dozen_to_count[&Dozen::Zero], 1);
        assert_eq!(half_to_count[&Half::One], 18);
        assert_eq!(half_to_count[&Half::Two], 18);
        assert_eq!(half_to_count[&Half::Zero], 1);
        assert_eq!(row_to_count[&Row::One], 12);
        assert_eq!(row_to_count[&Row::Two], 12);
        assert_eq!(row_to_count[&Row::Three], 12);
        assert_eq!(row_to_count[&Row::Zero], 1);
        assert_eq!(column_to_count.keys().len(), 13);
        assert_eq!(column_to_count[&Column::Zero], 1);
        column_to_count
            .iter()
            .filter(|&(&column, _)| column != Column::Zero)
            .for_each(|(_, &value)| {
                assert_eq!(value, 3);
            });
    }
    #[test]
    fn test_board_generation_american() {
        let result = Board::generate(&RouletteType::American);
        assert!(result.is_ok());
        let board = result.unwrap();
        let mut even_odd_to_count: HashMap<EvenOdd, usize> = HashMap::new();
        let mut color_to_count: HashMap<Color, usize> = HashMap::new();
        let mut dozen_to_count: HashMap<Dozen, usize> = HashMap::new();
        let mut half_to_count: HashMap<Half, usize> = HashMap::new();
        let mut row_to_count: HashMap<Row, usize> = HashMap::new();
        let mut column_to_count: HashMap<Column, usize> = HashMap::new();

        for slot in board.slots.iter() {
            even_odd_to_count
                .entry(slot.even_odd)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            color_to_count
                .entry(slot.color)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            dozen_to_count
                .entry(slot.dozen)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            half_to_count
                .entry(slot.half)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            row_to_count
                .entry(slot.row)
                .and_modify(|size| *size += 1)
                .or_insert(1);
            column_to_count
                .entry(slot.column)
                .and_modify(|size| *size += 1)
                .or_insert(1);
        }
        let num_of_slot_numbers = board.slots.len();
        let mut numbers: HashSet<SlotNumber> = HashSet::new();
        let mut has_dup = false;
        for slot in board.slots.iter() {
            assert!(slot.number >= -1 && slot.number <= 36);
            if numbers.contains(&slot.number) {
                has_dup = true;
                break;
            } else {
                numbers.insert(slot.number.clone());
            }
        }

        assert!(!has_dup);
        assert_eq!(num_of_slot_numbers, 38);
        assert_eq!(even_odd_to_count[&EvenOdd::Even], 18);
        assert_eq!(even_odd_to_count[&EvenOdd::Odd], 18);
        assert_eq!(even_odd_to_count[&EvenOdd::Zero], 2);
        assert_eq!(color_to_count[&Color::Red], 18);
        assert_eq!(color_to_count[&Color::Black], 18);
        assert_eq!(color_to_count[&Color::Green], 2);
        assert_eq!(dozen_to_count[&Dozen::One], 12);
        assert_eq!(dozen_to_count[&Dozen::Two], 12);
        assert_eq!(dozen_to_count[&Dozen::Three], 12);
        assert_eq!(dozen_to_count[&Dozen::Zero], 2);
        assert_eq!(half_to_count[&Half::One], 18);
        assert_eq!(half_to_count[&Half::Two], 18);
        assert_eq!(half_to_count[&Half::Zero], 2);
        assert_eq!(row_to_count[&Row::One], 12);
        assert_eq!(row_to_count[&Row::Two], 12);
        assert_eq!(row_to_count[&Row::Three], 12);
        assert_eq!(row_to_count[&Row::Zero], 2);
        assert_eq!(column_to_count.keys().len(), 13);
        assert_eq!(column_to_count[&Column::Zero], 2);
        column_to_count
            .iter()
            .filter(|&(&column, _)| column != Column::Zero)
            .for_each(|(_, &value)| {
                assert_eq!(value, 3);
            });
    }
}
