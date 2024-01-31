use crate::error::Error;

pub trait FromSlotNumber {
    type Output;
    fn from_slot_number(n: i64) -> Result<Self::Output, Error>;
}
