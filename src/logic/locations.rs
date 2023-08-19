use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display, strum::EnumIter, strum::EnumCount)]
pub enum Locations {}

impl Locations {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            _ => &[&[]],
        }
    }
}
