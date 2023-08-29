use super::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, strum::AsRefStr, strum::EnumIter, strum::EnumCount,
)]
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
