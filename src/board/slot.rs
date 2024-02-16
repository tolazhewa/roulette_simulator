use core::fmt;
use serde::{Deserialize, Serialize};

use crate::types::{
    color::Color, column::Column, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
    slot_number::SlotNumber,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct Slot {
    pub color: Color,
    pub number: SlotNumber,
    pub even_odd: EvenOdd,
    pub dozen: Dozen,
    pub half: Half,
    pub row: Row,
    pub column: Column,
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "Slot: {} {} {} {} {} {}",
            self.color, self.number, self.even_odd, self.dozen, self.half, self.row
        );
    }
}
