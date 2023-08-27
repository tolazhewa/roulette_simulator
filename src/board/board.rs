use super::slot::Slot;
use crate::{
    roulette::roulette_type::RouletteType,
    types::{color::Color, column::Column, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row},
};
use core::fmt;
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Board {
    pub slots: Vec<Slot>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Number of odds/evens
        let number_of_evens = self
            .slots
            .iter()
            .filter(|slot| slot.even_odd == EvenOdd::Even)
            .count();
        let number_of_odds = self
            .slots
            .iter()
            .filter(|slot| slot.even_odd == EvenOdd::Odd)
            .count();
        let number_of_even_odd_zeros = self
            .slots
            .iter()
            .filter(|slot| slot.even_odd == EvenOdd::Zero)
            .count();
        // Number of reds/blacks/greens
        let number_of_reds = self
            .slots
            .iter()
            .filter(|slot| slot.color == Color::Red)
            .count();
        let number_of_blacks = self
            .slots
            .iter()
            .filter(|slot| slot.color == Color::Black)
            .count();
        let number_of_greens = self
            .slots
            .iter()
            .filter(|slot| slot.color == Color::Green)
            .count();
        // Number of first/second/third dozen
        let number_of_first_dozen = self
            .slots
            .iter()
            .filter(|slot| slot.dozen == Dozen::One)
            .count();
        let number_of_second_dozen = self
            .slots
            .iter()
            .filter(|slot| slot.dozen == Dozen::Two)
            .count();
        let number_of_third_dozen = self
            .slots
            .iter()
            .filter(|slot| slot.dozen == Dozen::Three)
            .count();
        // Number of first/second half
        let number_of_first_half = self
            .slots
            .iter()
            .filter(|slot| slot.half == Half::One)
            .count();
        let number_of_second_half = self
            .slots
            .iter()
            .filter(|slot| slot.half == Half::Two)
            .count();
        // Ensure all numbers are covered
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
        writeln!(f, "\nBOARD PROPERTIES\n")?;
        writeln!(f, "Evens-Odds")?;
        writeln!(f, "------------------------")?;
        writeln!(f, "Number of evens: {}", number_of_evens)?;
        writeln!(f, "Number of odds: {}", number_of_odds)?;
        writeln!(f, "Number of zeros: {}\n", number_of_even_odd_zeros)?;
        writeln!(f, "Colors")?;
        writeln!(f, "------------------------")?;
        writeln!(f, "Number of reds: {}", number_of_reds)?;
        writeln!(f, "Number of blacks: {}", number_of_blacks)?;
        writeln!(f, "Number of greens: {}\n", number_of_greens)?;
        writeln!(f, "Dozens")?;
        writeln!(f, "------------------------")?;
        writeln!(f, "Number of first dozen: {}", number_of_first_dozen)?;
        writeln!(f, "Number of second dozen: {}", number_of_second_dozen)?;
        writeln!(f, "Number of third dozen: {}", number_of_third_dozen)?;
        writeln!(f, "Halves")?;
        writeln!(f, "------------------------")?;
        writeln!(f, "Number of first: {}", number_of_first_half)?;
        writeln!(f, "Number of second: {}", number_of_second_half)?;
        writeln!(f, "Numbers")?;
        writeln!(f, "------------------------")?;
        writeln!(f, "Number of slot number: {}", num_of_slot_numbers)?;
        writeln!(f, "All numbers are unique? {}\n", !has_dup)?;
        return write!(f, "------------------------\n\n");
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
