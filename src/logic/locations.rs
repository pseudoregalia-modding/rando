use super::*;

#[derive(Debug, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    DilapidatedDungeon,
    // CastleSansa,
    // Underbelly,
    // SansaKeep,
    // EmptyBailey,
    // ListlessLibrary,
    // TwilightTheatre,
    // TowerRuins,
    // FinalBoss,
}

use Location::*;

impl Location {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            DilapidatedDungeon => &[&[]],
        }
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            DilapidatedDungeon => "ZONE_Dungeon",
            // CastleSansa => "ZONE_LowerCastle",
            // Underbelly => "Zone_Caves",
            // SansaKeep => "Zone_Upper",
            // EmptyBailey => "ZONE_Exterior",
            // ListlessLibrary => "Zone_Library",
            // TwilightTheatre => "Zone_Theatre",
            // TowerRuins => "Zone_Tower",
            // FinalBoss => "Zone_PrincessChambers",
        }
    }
}
