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
    TheatreEntrance,
    OtherTheatrePath,
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
                    &[A::HeliacalPower, A::Sunsetter],
                    // this is possible with 2 precise successive solar wind flips and healing tech on pole
                    // &[A::Slide, A::SolarWind],
                ]),
            ]],
            L::Hole => &[&[
                Lock::Location(L::SansaKeep),
                Lock::Movement(&[&[A::HeliacalPower], &[A::SunGreaves], &[A::Sunsetter]]),
            ]],
            L::OtherTheatrePath => &[
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[&[A::AscendantLight], &[A::HeliacalPower], &[A::ClingGem]]),
                ],
                &[
                    Lock::Location(L::Prison),
                    Lock::Movement(&[
                        &[A::AscendantLight],
                        &[A::Sunsetter, A::HeliacalPower],
                        &[A::SunGreaves],
                        &[A::ClingGem],
                    ]),
                ],
            ],
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
                    Lock::Movement(&[
                        &[A::SunGreaves],
                        &[A::HeliacalPower],
                        &[A::ClingGem],
                        &[A::Sunsetter],
                    ]),
                ],
                &[Lock::Location(L::EmptyBailey)],
            ],
            L::TheatreEntrance => &[
                &[Lock::Location(L::MainTheatre)],
                &[
                    Lock::Location(L::CastleSansa),
                    Lock::Movement(&[
                        &[A::ClingGem],
                        &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
                        // this is hella precise but possible
                        // &[A::Slide, A::SolarWind, A::SunGreaves, A::Sunsetter],
                    ]),
                ],
            ],
            L::MainTheatre => &[
                &[
                    Lock::Location(L::TheatreEntrance),
                    Lock::Movement(&[
                        &[A::ClingGem],
                        &[A::SunGreaves],
                        &[A::Sunsetter, A::HeliacalPower],
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
                    Lock::Location(L::OtherTheatrePath),
                    Lock::Movement(&[
                        &[A::ClingGem, A::SunGreaves],
                        &[A::ClingGem, A::HeliacalPower],
                        &[A::Slide, A::SolarWind, A::ClingGem],
                    ]),
                ],
            ],
            L::FinalBoss => &[&[
                Lock::Location(L::TowerRuins),
                Lock::Movement(&[
                    &[A::SunGreaves, A::ClingGem],
                    &[A::HeliacalPower, A::Sunsetter, A::ClingGem],
                ]),
                Lock::Ending,
            ]],
        }
    }
    pub const fn file(&self) -> &'static str {
        match self {
            L::Prison | L::StrongEyes => "ZONE_Dungeon",
            L::CastleSansa => "ZONE_LowerCastle",
            L::MainLibrary | L::Restricted => "Zone_Library",
            L::SansaKeep | L::Sunsetter => "Zone_Upper",
            L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuins => "Zone_Tower",
            L::Hole | L::MainUnderbelly => "Zone_Caves",
            L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Zone_Theatre"
            }
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::Prison | L::StrongEyes => "Dilapidated Dungeon",
            L::CastleSansa => "Castle Sansa",
            L::MainLibrary | L::Restricted => "Listless Library",
            L::SansaKeep | L::Sunsetter => "Sansa Keep",
            L::EmptyBailey => "Empty Bailey",
            L::TowerRuins => "Tower Ruins",
            L::Hole | L::MainUnderbelly => "Underbelly",
            L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Twilight Theatre"
            }
            L::FinalBoss => "Princess",
        }
    }
}
