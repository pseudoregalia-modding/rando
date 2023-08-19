use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display, strum::EnumIter, strum::EnumCount)]
pub enum Locations {
    #[strum(serialize = "ZONE_Dungeon")]
    DilapidatedDungeon,
}

impl Locations {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            _ => &[&[]],
        }
    }
}
