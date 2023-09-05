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
    Hole,
    MainUnderbelly,
    PillarRoom,
    MainTheatre,
    FinalBoss,
}

use Ability as A;
use Location as L;

impl Location {
    // need to include some reverse directions
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            L::Prison => &[],
            L::StrongEyes => &[&[Lock::Location(L::Prison), Lock::Movement(&[&[A::Slide]])]],
            L::CastleSansa => &[
                &[Lock::Location(L::StrongEyes), Lock::SmallKey],
                &[Lock::Location(L::EmptyBailey)],
            ],
            L::MainLibrary => &[&[Lock::Location(L::CastleSansa)]],
            L::Restricted => &[&[Lock::Location(L::MainLibrary), Lock::SmallKey]],
            L::SansaKeep => &[
                &[Lock::Location(L::CastleSansa)],
                &[
                    Lock::Location(L::MainTheatre),
                    Lock::Movement(&[&[A::ClingGem]]),
                ],
            ],
            L::Sunsetter => &[
                &[Lock::Location(L::SansaKeep), Lock::SmallKey],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]]),
                ],
            ],
            L::EmptyBailey => &[
                &[Lock::Location(L::CastleSansa)],
                &[Lock::Location(L::MainUnderbelly)],
            ],
            L::TowerRuins => &[&[
                Lock::Location(L::EmptyBailey),
                Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Slide, A::Sunsetter],
                    // to actually get into the tower you could abuse solar wind flips but that's advanced af
                ]),
            ]],
            L::Hole => &[&[
                Lock::Location(L::SansaKeep),
                Lock::Movement(&[&[A::HeliacalPower], &[A::SunGreaves], &[A::Sunsetter]]),
            ]],
            L::MainUnderbelly => &[
                &[Lock::Location(L::Prison)],
                // just so I don't need to rewrite conditions
                &[
                    Lock::Location(L::TowerRuins),
                    Lock::Movement(&[&[A::Sunsetter]]),
                ],
                &[Lock::Location(L::Hole), Lock::Movement(&[&[A::Sunsetter]])],
            ],
            L::PillarRoom => &[
                // this is via the entrance above the normal entrance but needs some moar to get in maybe make separate
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[&[A::SunGreaves], &[A::HeliacalPower], &[A::ClingGem]]),
                ],
                &[Lock::Location(L::EmptyBailey)],
            ],
            L::MainTheatre => &[
                // this is via the softlock entrance
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[
                        &[A::ClingGem],
                        &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
                    ]),
                ],
                &[
                    Lock::Location(L::PillarRoom),
                    Lock::Movement(&[
                        &[A::Sunsetter, A::ClingGem],
                        &[A::Sunsetter, A::SunGreaves, A::HeliacalPower],
                    ]),
                ],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[
                        &[A::ClingGem, A::SunGreaves],
                        &[A::Slide, A::SolarWind, A::ClingGem],
                    ]),
                ],
                &[
                    Lock::Location(L::Prison),
                    Lock::Movement(&[
                        &[A::SunGreaves],
                        &[A::ClingGem],
                        &[A::Slide, A::SolarWind, A::HeliacalPower],
                        &[A::AscendantLight, A::HeliacalPower],
                    ]),
                ],
            ],
            L::FinalBoss => &[&[
                Lock::Location(L::TowerRuins),
                Lock::Movement(&[&[A::SunGreaves, A::ClingGem]]),
                Lock::Ending,
            ]],
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
            L::Hole | L::MainUnderbelly => "Zone_Caves",
            L::PillarRoom | L::MainTheatre => "Zone_Theatre",
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
}
