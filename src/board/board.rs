use super::slot::Slot;
use crate::{
    roulette::roulette_type::RouletteType,
    types::{color::Color, column::Column, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row},
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
        let mut numbers: HashSet<String> = HashSet::new();
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
    pub fn generate(roulette_type: &RouletteType) -> Self {
        let mut rng = rand::thread_rng();
        let mut slots: Vec<Slot> = Vec::new();
        slots.push(Slot {
            color: Color::Green,
            number: String::from("0"),
            even_odd: EvenOdd::Zero,
            dozen: Dozen::Zero,
            half: Half::Zero,
            row: Row::Zero,
            column: Column::Zero,
        });
        if *roulette_type == RouletteType::American {
            slots.push(Slot {
                color: Color::Green,
                number: String::from("00"),
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
            let color: Color = get_color(&color_counter, &mut rng);
            color_counter.entry(color).and_modify(|count| *count -= 1);

            let even_odd: EvenOdd = get_even_odd(n);
            let number: String = n.to_string();
            let dozen: Dozen = get_dozen(n);
            let half: Half = get_half(n);
            let row: Row = get_row(n);
            let column: Column = get_column(n);
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
        return Board { slots };
    }
}

fn get_color(color_counter: &HashMap<Color, i32>, rng: &mut ThreadRng) -> Color {
    let color: Color;
    if color_counter[&Color::Red] <= 0 {
        color = Color::Black;
    } else if color_counter[&Color::Black] <= 0 {
        color = Color::Red;
    } else {
        color = if rng.gen::<f64>() > 0.5 {
            Color::Red
        } else {
            Color::Black
        };
    }
    return color;
}

fn get_even_odd(n: i32) -> EvenOdd {
    if n % 2 == 0 {
        return EvenOdd::Even;
    } else {
        return EvenOdd::Odd;
    }
}

fn get_dozen(n: i32) -> Dozen {
    let dozen: Dozen;
    if n >= 1 && n <= 12 {
        dozen = Dozen::One;
    } else if n >= 13 && n <= 24 {
        dozen = Dozen::Two;
    } else if n >= 25 && n <= 36 {
        dozen = Dozen::Three;
    } else {
        dozen = Dozen::Zero;
    }
    return dozen;
}

fn get_half(n: i32) -> Half {
    let half: Half;
    if n >= 1 && n <= 18 {
        half = Half::One;
    } else if n >= 19 && n <= 36 {
        half = Half::Two;
    } else {
        half = Half::Zero;
    }
    return half;
}

fn get_row(n: i32) -> Row {
    let row: Row;
    if n % 3 == 0 {
        row = Row::One;
    } else if n % 3 == 1 {
        row = Row::Two;
    } else if n % 3 == 2 {
        row = Row::Three;
    } else {
        row = Row::Zero;
    }
    return row;
}

fn get_column(n: i32) -> Column {
    return Column::from_number(((n - 1) / 3) + 1);
}
