use super::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    Prison,
    StrongEyes,
    CastleSansaMain,
    CastleSansaTheatreEntrance,
    MainLibrary,
    Restricted,
    SansaKeep,
    Sunsetter,
    EmptyBailey,
    TowerRuinsEntrance,
    TowerRuinsKeep,
    SansaHole,
    BaileyHole,
    PrisonHole,
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
            // Prison / Dilapidated Dungeon
            L::Prison => &[
                &[Lock::Location(L::PrisonHole), Lock::Movement(&[
                    &[A::DreamBreaker, A::AscendantLight] // Climb the poles and break the wall in Prison
                ])],// Enter from Underbelly
                &[Lock::Location(L::CastleSansaMain)],
            ],
            L::StrongEyes => &[
                &[Lock::Location(L::Prison), Lock::Movement(&[&[A::Slide]])],
                &[Lock::Location(L::CastleSansaMain), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
            ],
            // Castle Sansa
            L::CastleSansaMain => &[
                &[Lock::Location(L::StrongEyes), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::EmptyBailey)],
                &[Lock::Location(L::CastleSansaTheatreEntrance), Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::Slide, &[ &[A::HeliacalPower], &[A::SunGreaves]]],
                    &[A::Slide, A::SolarWind],
                ])],
                &[LockLocation(L::MainLibrary), Lock::Movement(&[
                    &[A::DreamBreaker],
                ])]
            ],
            L::CastleSansaTheatreEntrance => &[
                &[Lock::Location(L::TheatreEntrance)],
                &[Lock::Location(L::CastleSansaMain), Lock::Movement(&[
                    &[
                        &[A::Sunsetter, &[ &[A::Helical], &[A::SunGreaves] ], A::ClingGem]
                    ]
                ])]
            ],
            // Library
            L::MainLibrary => &[&[Lock::Location(L::CastleSansaMain), Lock::Movement(&[
                &[A::DreamBreaker],
            ])]],
            L::Restricted => &[&[Lock::Location(L::MainLibrary), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]]), Lock::Movement(&[&[A::DreamBreaker]])]],
            // Sansa Keep
            L::SansaKeep => &[
                &[Lock::Location(L::CastleSansaMain)],
                &[
                    Lock::Location(L::MainTheatre),
                    Lock::Movement(&[&[A::ClingGem]]),
                ],
                &[Lock::Location(L::SansaHole), Lock::Movement(&[
                    &[A::Sunsetter],
                    &[A::HeliacalPower],
                    &[A::SunGreaves],
                    &[A::Slide, A::SolarWind],
                    
                ])],
            ],
            L::Sunsetter => &[
                &[Lock::Location(L::SansaKeep), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
                &[
                    Lock::Location(L::SansaKeep),
                    Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]]),
                ],
            ],
            // Bailey
            L::EmptyBailey => &[
                &[Lock::Location(L::CastleSansaMain)],
                &[Lock::Location(L::BaileyHole)],
                &[Lock::Location(L::PillarRoom)], // Enter from Theatre's Floor is lava/pillar room.
                &[Lock::Location(L::TowerRuinsEntrance), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::Sunsetter],
                    &[A::ClingGem, &[ &[ A::HeliacalPower], &[A::SunGreaves]]],
                    &[A::Slide, A::SolarWind],
                ])],
            ],
            // Tower
            L::TowerRuinsEntrance => &[
                &[
                    Lock::Location(L::EmptyBailey),
                    Lock::Movement(&[
                        &[A::SunGreaves],
                        &[A::Sunsetter],
                        &[A::ClingGem, &[ &[ A::HeliacalPower], &[A::SunGreaves]]],
                        &[A::Slide, A::SolarWind],
                    ]),
                ],
                &[Lock::Location(L::TowerRuinsKeep)],
            ],
            L::TowerRuinsKeep => &[
                &[Lock::Location(L::TowerRuinsEntrance), Lock::Movement(&[
                    &[A::Sunsetter, A::ClingGem],
                    &[A::ClingGem, A::HeliacalPower],
                    &[A::SunGreaves],
                    // Test the below to make sure not miss remembering.
                    // &[A::Slide, A::SolarWind, A::HeliacalPower],
                ])],
                &[
                    Lock::Location(L::FinalBoss)
                ]
            ],
            // Underbelly
            L::PrisonHole => &[
                &[
                    Lock::Location(L::Prison),
                    Lock::Movement(&[&[A::DreamBreaker], &[A::Sunsetter] ])// Dream breaker or Sunsetter to enter.
                ],
                &[
                    Lock::Location(L::MainUnderbelly), // From main to the hole (right below the gear mobs.)
                    Lock::Movement(&[
                        &[
                            &[A::SunGreaves],
                            &[A::Sunsetter],
                            &[A::Slide, A::SolarWind],
                        ]
                    ])
                ]
            ],
            L::BaileyHole => &[
                &[
                    Lock::Location(L::TowerRuinsEntrance),
                    Lock::Movement(&[&[A::Sunsetter]]), // From Bailey into underbelly.
                ],
                &[
                    Lock::Location(L::MainUnderbelly), // From main to hole.
                ],
                &[
                    Lock::Location(L::SansaHole),
                    Lock::Movement(&[
                        &[A::Sunsetter],
                    ])
                ],
            ],
            L::SansaHole => &[
                &[
                    Lock::Location(L::SansaKeep), // From Sansa keep into underbelly
                    Lock::Movement(&[&[A::HeliacalPower], &[A::SunGreaves], &[A::Sunsetter]]),
                ],
                &[
                    Lock::Location(L::MainUnderbelly), // From main underbelly to the hole.
                    Lock::Movement(&[
                        &[A::Sunsetter],
                    ])
                ],
                &[
                    Lock::Location(L::BaileyHole),
                    Lock::Movement(&[
                        &[A::Sunsetter],
                    ])
                ],
            ],
            L::MainUnderbelly => &[ // Main underbelly is now the main platform and any check possible from it. Helical power check will be combined with this.
                &[Lock::Location(L::PrisonHole)],
                &[Lock::Location(L::BaileyHole), Lock::Movement(&[
                    &[A::SunGreaves, A::HeliacalPower, A::Sunsetter], // Going from first bubble to the circular platform
                    &[
                        &[&[A::Sunsetter], &[A::HeliacalPower], &[A::SunGreaves]], // Sunsetter OR Helical OR Sungreaves
                        A::Slide
                    ], // Slide + the Nested OR. Goes through the Helical power route.
                ])]
                &[Lock::Location(L::SansaHole), Lock::Movement(&[&[A::Sunsetter, A::Slide]])], // from Sansa hole (above going to Major Key)
            ],
            //Theatre
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
            L::PillarRoom => &[
                // this is via the entrance above the normal entrance but needs some moar to get in maybe make separate
                &[
                    Lock::Location(L::CastleSansaMain),
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
                    Lock::Location(L::CastleSansaMain),
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
            // Final Boss
            L::FinalBoss => &[&[
                Lock::Location(L::TowerRuinsEntrance),
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
            L::CastleSansaMain | L::CastleSansaTheatreEntrance => "ZONE_LowerCastle",
            L::MainLibrary | L::Restricted => "Zone_Library",
            L::SansaKeep | L::Sunsetter => "Zone_Upper",
            L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuinsEntrance | L::TowerRuinsKeep  => "Zone_Tower",
            L::SansaHole |L::PrisonHole| L::BaileyHole| L::MainUnderbelly => "Zone_Caves",
            L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Zone_Theatre"
            }
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::Prison | L::StrongEyes => "Dilapidated Dungeon",
            L::CastleSansaMain | L::CastleSansaTheatreEntrance => "Castle Sansa",
            L::MainLibrary | L::Restricted => "Listless Library",
            L::SansaKeep | L::Sunsetter => "Sansa Keep",
            L::EmptyBailey => "Empty Bailey",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Tower Ruins",
            L::SansaHole | L::PrisonHole |L::BaileyHole| L::MainUnderbelly => "Underbelly",
            L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Twilight Theatre"
            }
            L::FinalBoss => "Princess",
        }
    }
}
