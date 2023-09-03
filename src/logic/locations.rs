use super::*;

#[derive(Debug, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    Prison,
    StrongEyes,
    CastleSansa,
    Underbelly,
    // SansaKeep,
    MainLibrary,
    Restricted,
    EmptyBailey,
    // TwilightTheatre,
    TowerRuins,
    // FinalBoss,
}

use Location as L;

impl Location {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            L::Prison => &[],
            L::StrongEyes => &[&[Lock::Location(L::Prison), Lock::Movement(&[Ability::Slide])]],
            L::CastleSansa => &[&[Lock::Location(L::StrongEyes), Lock::SmallKey]],
            L::MainLibrary => &[&[Lock::Location(L::CastleSansa)]],
            L::Restricted => &[&[Lock::Location(L::MainLibrary), Lock::SmallKey]],
            L::Underbelly => &[&[Lock::Location(L::Prison)]],
            L::EmptyBailey => &[&[Lock::Location(L::CastleSansa)]],
            L::TowerRuins => &[&[
                Lock::Location(Location::EmptyBailey),
                Lock::Movement(&[
                    &[Ability::SunGreaves],
                    &[Ability::HeliacalPower],
                    &[Ability::ClingGem],
                    &[Ability::Slide, Ability::Sunsetter],
                ]),
            ]],
        }
    }
    pub const fn as_str(&self) -> &str {
        match self {
            L::Prison | L::StrongEyes => "ZONE_Dungeon",
            L::CastleSansa => "ZONE_LowerCastle",
            L::MainLibrary | L::Restricted => "Zone_Library",
            L::Underbelly => "Zone_Caves",
            // L::SansaKeep => "Zone_Upper",
            L::EmptyBailey => "ZONE_Exterior",
            // L::TwilightTheatre => "Zone_Theatre",
            L::TowerRuins => "Zone_Tower",
            // L::FinalBoss => "Zone_PrincessChambers",
        }
    }
}
