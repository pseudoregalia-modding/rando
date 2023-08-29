use super::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, strum::AsRefStr, strum::EnumIter, strum::EnumCount,
)]
pub enum Location {
    #[strum(serialize = "ZONE_Dungeon")]
    DilapidatedDungeon,
    #[strum(serialize = "ZONE_LowerCastle")]
    CastleSansa,
    #[strum(serialize = "Zone_Caves")]
    Underbelly,
    #[strum(serialize = "Zone_Upper")]
    SansaKeep,
    #[strum(serialize = "ZONE_Exterior")]
    EmptyBailey,
    #[strum(serialize = "Zone_Library")]
    ListlessLibrary,
    #[strum(serialize = "Zone_Theatre")]
    TwilightTheatre,
    #[strum(serialize = "Zone_Tower")]
    TowerRuins,
    #[strum(serialize = "Zone_PrincessChambers")]
    FinalBoss,
}

impl Location {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            _ => &[&[]],
        }
    }
}
