use super::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    Prison,
    StrongEyes,
    CastleSansa,
    MainLibrary,
    Restricted,
    SansaKeep,
    Sunsetter,
    EmptyBailey,
    TowerRuins,
    Underbelly,
    PillarRoom,
    MainTheatre,
    // FinalBoss,
}

use Ability as A;
use Location as L;

impl Location {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            L::Prison => &[],
            L::StrongEyes => &[&[Lock::Location(L::Prison), Lock::Movement(&[A::Slide])]],
            L::CastleSansa => &[&[Lock::Location(L::StrongEyes), Lock::SmallKey]],
            L::MainLibrary => &[&[Lock::Location(L::CastleSansa)]],
            L::Restricted => &[&[Lock::Location(L::MainLibrary), Lock::SmallKey]],
            L::SansaKeep => &[&[Lock::Location(L::CastleSansa)]],
            L::Sunsetter => &[
                &[Lock::Location(L::SansaKeep), Lock::SmallKey],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[A::SunGreaves]),
                ],
                &[Lock::Location(L::SansaKeep), Lock::Movement(&[A::ClingGem])],
            ],
            L::EmptyBailey => &[&[Lock::Location(L::CastleSansa)]],
            L::TowerRuins => &[
                &[
                    Lock::Location(L::EmptyBailey),
                    Lock::Movement(&[A::SunGreaves]),
                ],
                &[
                    Lock::Location(L::EmptyBailey),
                    Lock::Movement(&[A::HeliacalPower]),
                ],
                &[
                    Lock::Location(L::EmptyBailey),
                    Lock::Movement(&[A::ClingGem]),
                ],
                &[
                    Lock::Location(L::EmptyBailey),
                    Lock::Movement(&[A::Slide, A::Sunsetter]),
                ],
                // to actually get into the tower you could abuse solar wind flips but that's advanced af
            ],
            L::Underbelly => &[&[Lock::Location(L::Prison)]],
            L::PillarRoom => &[
                // this is via the entrance above the normal entrance but needs some moar to get in maybe make separate
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[A::SunGreaves]),
                ],
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[A::HeliacalPower]),
                ],
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[A::ClingGem]),
                ],
                &[Lock::Location(L::EmptyBailey)],
            ],
            L::MainTheatre => &[
                // this is via the softlock entrance
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[A::ClingGem]),
                ],
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower]),
                ],
                &[
                    Lock::Location(L::PillarRoom),
                    Lock::Movement(&[A::Sunsetter, A::ClingGem]),
                ],
                &[
                    Lock::Location(L::PillarRoom),
                    Lock::Movement(&[A::Sunsetter, A::SunGreaves, A::HeliacalPower]),
                ],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[A::ClingGem, A::SunGreaves]),
                ],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[A::Slide, A::SolarWind, A::ClingGem]),
                ],
                &[
                    Lock::Location(L::Prison),
                    Lock::Movement(&[A::ClingGem, A::SunGreaves]),
                ],
                &[
                    Lock::Location(L::Prison),
                    Lock::Movement(&[A::Slide, A::SolarWind, A::ClingGem]),
                ],
            ],
        }
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            L::Prison | L::StrongEyes => "ZONE_Dungeon",
            L::CastleSansa => "ZONE_LowerCastle",
            L::MainLibrary | L::Restricted => "Zone_Library",
            L::SansaKeep | L::Sunsetter => "Zone_Upper",
            L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuins => "Zone_Tower",
            L::Underbelly => "Zone_Caves",
            L::PillarRoom | L::MainTheatre => "Zone_Theatre",
            // L::FinalBoss => "Zone_PrincessChambers",
        }
    }
}
