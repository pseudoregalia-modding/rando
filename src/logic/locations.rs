use super::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    //Prison
    VDreamBreaker,
    EarlyPrison,
    LatePrison,
    StrongEyes,
    PEntryCastle,
    PEntryUnderBelly,
    PEntryTheatre,
    //Castle
    CsOldSoftlockRoom,
    CsKeepClimbEntrance,
    CsMain,
    CsTheatreEntrance,
    CsPrisonEntry,
    CsLibraryEntry,
    CsTheatreEntryNearPrison,
    CsKeepEntryMain,
    CsKeepEntryRamp,
    CsBaileyEntry,
    //Library
    MainLibrary,
    Restricted,
    LibSaveNearGreaves,
    //Keep
    SkCastleRampEntry,
    SkCastleMainEntry,
    SkCastleClimbEntry,
    SkUnderbellyEntry,
    SkTheatreEntry,
    SansaKeep,
    Sunsetter,
    //Bailey
    EmptyBailey,
    EbEntryUnderBelly,
    EbEntryRuins,
    EbEntryTheatre,
    EbEntryCastle,
    //Ruins
    TowerRuinsEntrance,
    TowerRuinsKeep,
    //Underbelly
    SansaHole,
    BaileyHole,
    PrisonHole,
    MainUnderbelly,
    VAscendantLight,
    HpSave,
    //Theatre
    ThCastleEntryPillar,
    ThCastleEntryMain,
    ThBaileyEntry,
    ThKeepEntry,
    ThDungeonEntry,
    PillarRoom,
    TheatreEntrance,
    OtherTheatrePath,
    MainTheatre,
    //Final
    FinalBoss,
}

use Ability as A;
use Location as L;

impl Location {
    // need to include some reverse directions
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            // Prison / Dilapidated Dungeon
            L::LatePrison => &[
                &[Lock::Location(L::PEntryUnderBelly), Lock::Movement(&[&[A::DreamBreaker]])],// Enter from Underbelly
                &[Lock::Location(L::CsMain)],
                &[Lock::Location(L::EarlyPrison), Lock::Movement(&[&[A::DreamBreaker]])],
            ],
            L::EarlyPrison => &[
                &[Lock::Location(L::StrongEyes), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::CsMain)],
                &[Lock::Location(L::VDreamBreaker), Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::LatePrison), Lock::Movement(&[&[A::DreamBreaker]])]
            ],
            L::PEntryUnderBelly => &[
                &[Lock::Location(L::LatePrison), Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::PrisonHole),  Lock::Movement(&[
                    &[A::DreamBreaker, A::AscendantLight] // Climb the poles and break the wall in LatePrison
                ])],
            ],
            L::VDreamBreaker => &[
                &[Lock::Location(L::EarlyPrison)]
            ],
            L::StrongEyes => &[
                &[Lock::Location(L::LatePrison), Lock::Movement(&[&[A::Slide]])],
                &[Lock::Location(L::CsMain), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
            ],
            L::PEntryCastle => &[
                &[Lock::Location(L::StrongEyes), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::CsPrisonEntry)],
            ],
            L::PEntryTheatre => &[
                &[Lock::Location(L::LatePrison), Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::AscendantLight]
                ])],
                &[Lock::Location(L::ThDungeonEntry)],
            ],
            // Castle Sansa
            L::CsPrisonEntry => &[
                &[Lock::Location(L::CsMain)],
                &[Lock::Location(L::PEntryCastle)],
            ],
            L::CsLibraryEntry => &[
                &[Lock::Location(L::CsMain), Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::MainLibrary)],
            ],
            L::CsTheatreEntryNearPrison => &[
                &[Lock::Location(L::CsMain), Lock::Movement(&[&[A::SunGreaves], &[A::Sunsetter], &[A::ClingGem], &[A::Slide, A::SolarWind]])],
                &[Lock::Location(L::PillarRoom)],
            ],
            L::CsOldSoftlockRoom => &[
                &[Lock::Location(L::CsMain), Lock::Movement(&[&[A::ClingGem]])],
                &[Lock::Location(L::CsTheatreEntrance), Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::Slide, A::HeliacalPower],
                    &[A::Slide, A::SunGreaves],
                    &[A::Slide, A::SolarWind],
                ])],
            ],
            L::CsKeepClimbEntrance => &[
                &[Lock::Location(L::CsMain), Lock::SmallKey],
            ],
            L::CsKeepEntryMain => &[
                &[Lock::Location(L::CsMain)],
                &[Lock::Location(L::SansaKeep)],
            ],
            L::CsKeepEntryRamp => &[
                &[Lock::Location(L::CsMain), Lock::Movement(&[&[A::DreamBreaker], &[A::ClingGem], &[A::SunGreaves], &[A::Sunsetter]])],
                &[Lock::Location(L::SansaKeep)],
            ],
            L::CsBaileyEntry => &[
                &[Lock::Location(L::CsMain)],
                &[Lock::Location(L::EbEntryCastle)],
            ],
            L::CsMain => &[
                &[Lock::Location(L::CsKeepClimbEntrance), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]])],
                &[Lock::Location(L::CsPrisonEntry)],
                &[Lock::Location(L::CsBaileyEntry)],
                &[Lock::Location(L::CSOldSoftlockRoom), Lock::Movement(&[
                    &[A::ClingGem],
                ])],
                &[Lock::Location(L::CsLibraryEntry), Lock::Movement(&[
                    &[A::DreamBreaker],
                ])],
                &[Lock::Location(L::CsTheatreEntryNearPrison)],
            ],
            L::CsTheatreEntrance => &[
                &[Lock::Location(L::ThCastleEntryMain)],
                &[Lock::Location(L::CSOldSoftlockRoom), Lock::Movement(&[
                    &[A::Sunsetter, A::HeliacalPower, A::ClingGem],
                    &[A::Sunsetter, A::SunGreaves, A::ClingGem],
                ])]
            ],
            // Library
            L::LibSaveNearGreaves => &[
                &[Lock::Location(L::MainLibrary), Lock::Movement(&[
                    //Entry from Sungreaves route..
                    &[A::DreamBreaker, A::Slide, A::SolarWind],
                    &[A::DreamBreaker, A::Slide, A::Sunsetter],
                    &[A::DreamBreaker, A::Slide, A::SunGreaves],
                    //Entry from Reverse..
                    &[A::SunGreaves],
                    &[A::ClingGem, A::HeliacalPower],
                    &[A::ClingGem, A::Sunsettern],
                    &[A::Slide, A::SolarWind, A::HeliacalPower],
                ])],
            ],
            L::MainLibrary => &[
                &[Lock::Location(L::CsMain), Lock::Movement(&[
                    &[A::DreamBreaker],
                ])],
                &[Lock::Location(L::LibSaveNearGreaves), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::ClingGem, A::HeliacalPower],
                    &[A::ClingGem, A::Sunsettern],
                    &[A::Slide, A::SolarWind, A::HeliacalPower],
                    &[A::DreamBreaker, A::HeliacalPower],
                ])],
            ],
            L::Restricted => &[&[Lock::Location(L::MainLibrary), Lock::SmallKey, Lock::Movement(&[&[A::DreamBreaker]]), Lock::Movement(&[&[A::DreamBreaker]])]],
            // Sansa Keep
            L::SkCastleClimbEntry => &[
                &[Lock::Location(L::CsKeepClimbEntrance)],
            ],
            L::SkCastleMainEntry => &[
                &[Lock::Location(L::SansaKeep)],
                &[Lock::Location(L::CsKeepEntryMain)],
            ],
            L::SkCastleRampEntry => &[
                &[Lock::Location(L::SansaKeep), Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::Slide, A::SolarWind],
                ])],
                &[Lock::Location(L::CsKeepEntryRamp)],
            ],
            L::SkUnderbellyEntry => &[
                &[Lock::Location(L::SansaKeep), Lock::Movement(&[
                    &[A::Sunsetter],
                    &[A::HeliacalPower],
                    &[A::SunGreaves],
                    &[A::Slide, A::SolarWind],    
                ])],
                &[Lock::Location(L::SansaHole)],
            ],
            L::SkTheatreEntry => &[
                &[Lock::Location(L::ThKeepEntry)],
                &[Lock::Location(L::SansaKeep), Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::Slide, A::SolarWind]
                ])]
            ],
            L::SansaKeep => &[
                &[Lock::Location(L::SkCastleRampEntry)],
                &[Lock::Location(L::SkCastleMainEntry)],
                &[
                    Lock::Location(L::MainTheatre),
                    Lock::Movement(&[&[A::ClingGem]]),
                ],
                &[Lock::Location(L::SkUnderbellyEntry), Lock::Movement(&[
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
            L::EbEntryCastle => &[
                &[Lock::Location(L::CsBaileyEntry)],
                &[Lock::Location(L::EmptyBailey)],
            ],
            L::EbEntryRuins => &[
                &[Lock::Location(L::TowerRuinsEntrance), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::Sunsetter],
                    &[A::ClingGem, A::HeliacalPower],
                    &[A::Slide, A::SolarWind],
                ])],
                &[Lock::Location(L::EmptyBailey), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::Sunsetter],
                    &[A::ClingGem, A::HeliacalPower],
                    &[A::Slide, A::SolarWind],
                ])],
            ],
            L::EbEntryTheatre => &[
                &[Lock::Location(L::EmptyBailey)],
                &[Lock::Location(L::PillarRoom)],
            ],
            L::EbEntryUnderBelly => &[
                &[Lock::Location(L::BaileyHole)],
                &[Lock::Location(L::EmptyBailey), Lock::Movement(&[
                    &[A::Sunsetter],
                    &[A::Slide, A::SolarWind],
                    &[A::SunGreaves],
                    &[A::HeliacalPower]
                ])],
            ],
            L::EmptyBailey => &[
                &[Lock::Location(L::EbEntryCastle)],
                &[Lock::Location(L::EbEntryUnderBelly)],
                &[Lock::Location(L::EbEntryTheatre)],
                &[Lock::Location(L::EbEntryRuins), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::Sunsetter],
                    &[A::ClingGem, A::HeliacalPower],
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
                        &[A::ClingGem, A::HeliacalPower],
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
                    Lock::Location(L::PEntryUnderBelly),
                    Lock::Movement(&[&[A::DreamBreaker] ])
                ],
                &[
                    Lock::Location(L::MainUnderbelly), // From main to the hole (right below the gear mobs.)
                    Lock::Movement(&[
                        &[A::SunGreaves],
                        &[A::Sunsetter],
                        &[A::Slide, A::SolarWind],
                    ])
                ]
            ],
            L::BaileyHole => &[
                &[
                    Lock::Location(L::EbEntryUnderBelly),
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
                    Lock::Location(L::SkUnderbellyEntry), // From Sansa keep into underbelly
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
                &[
                    Lock::Location(L::HpSave),
                    Lock::Movement(&[
                        A::Sunsetter,
                        A::Slide,
                    ])
                ],
            ],
            L::MainUnderbelly => &[ // Main underbelly is now the main platform and any check possible from it. Helical power check will be combined with this.
                &[Lock::Location(L::PrisonHole)],
                &[Lock::Location(L::BaileyHole), Lock::Movement(&[
                    &[A::SunGreaves, A::HeliacalPower, A::Sunsetter], // Going from first bubble to the circular platform
                ])],
                &[Lock::Location(L::HpSave), Lock::Movement(&[
                    &[A::Slide, A::SunGreaves, A::Slide, A::SolarWind, A::ClingGem], // Slide through top of gap and ultra out of solar into wall kick up.
                    &[A::DreamBreaker, A::ClingGem, A::Sunsetter],
                    &[A::DreamBreaker, A::HeliacalPower, A::SunGreaves],
                    &[A::DreamBreaker, A::Slide, A::SolarWind, A::HeliacalPower],
                ])],
                &[Lock::Location(L::SansaHole), Lock::Movement(&[&[A::Sunsetter, A::Slide]])], // from Sansa hole (above going to Major Key)
            ],
            L::VAscendantLight => &[
                &[Lock::Location(L::PrisonHole), Lock::Movement(&[
                    &[A::DreamBreaker],
                ])],
            ],
            L::HpSave => &[
                &[Lock::Location(L::BaileyHole), Lock::Movement(&[
                    &[A::Slide, A::SunGreaves,],
                    &[A::Slide, A::Sunsetter,],
                    &[A::Slide, A::HeliacalPower,],
                ])],
                &[Lock::Location(L::MainUnderbelly), Lock::Movement(&[
                    &[A::DreamBreaker, A::ClingGem, A::Sunsetter],
                    &[A::DreamBreaker, A::HeliacalPower, A::SunGreaves],
                    &[A::DreamBreaker, A::Slide, A::SolarWind, A::HeliacalPower],
                ])],
            ],
            //Theatre
            L::ThBaileyEntry => &[
                &[Lock::Location(L::EbEntryTheatre)],
                &[Lock::Location(L::PillarRoom), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Sunsetter],
                ])],
            ],
            L::ThCastleEntryMain => &[
                &[Lock::Location(L::CsTheatreEntrance)],
                &[Lock::Location(L::TheatreEntrance)],
            ],
            L::ThCastleEntryPillar => &[
                &[Lock::Location(L::CsTheatreEntryNearPrison)],
                &[Lock::Location(L::PillarRoom), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Sunsetter],
                ])],
            ],
            L::ThKeepEntry => &[
                &[Lock::Location(L::SkTheatreEntry)],
                &[Lock::Location(L::OtherTheatrePath), 
                    Lock::Movement(&[
                        &[A::AscendantLight, A::DreamBreaker], &[A::HeliacalPower], &[A::ClingGem]
                    ])
                ],
            ],
            L::ThDungeonEntry => &[
                &[Lock::Location(L::OtherTheatrePath), Lock::Movement(&[
                    &[A::AscendantLight, A::DreamBreaker],
                    &[A::Sunsetter, A::HeliacalPower],
                    &[A::SunGreaves],
                    &[A::ClingGem],
                ])],
                &[Lock::Location(L::PEntryTheatre)],
            ],
            L::OtherTheatrePath => &[
                &[
                    Lock::Location(L::ThKeepEntry),
                    Lock::Movement(&[&[A::AscendantLight, A::DreamBreaker], &[A::HeliacalPower], &[A::ClingGem]]),
                ],
                &[
                    Lock::Location(L::ThDungeonEntry),
                    Lock::Movement(&[
                        &[A::AscendantLight, A::DreamBreaker],
                        &[A::Sunsetter, A::HeliacalPower],
                        &[A::SunGreaves],
                        &[A::ClingGem],
                    ]),
                ],
            ],
            L::PillarRoom => &[
                // this is via the entrance above the normal entrance but needs some moar to get in maybe make separate
                &[
                    Lock::Location(L::ThCastleEntryPillar),
                    Lock::Movement(&[
                        &[A::SunGreaves],
                        &[A::HeliacalPower],
                        &[A::ClingGem],
                        &[A::Sunsetter],
                    ]),
                ],
                &[Lock::Location(L::ThBaileyEntry), Lock::Movement(&[
                    &[A::SunGreaves],
                    &[A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Sunsetter],
                ])],
            ],
            L::TheatreEntrance => &[
                &[Lock::Location(L::MainTheatre)],
                &[
                    Lock::Location(L::ThCastleEntryMain),
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
            L::PEntryTheatre | L::PEntryCastle| L::PEntryUnderBelly | L::LatePrison | L::VDreamBreaker | L::EarlyPrison| L::StrongEyes => "ZONE_Dungeon",
            L::CsBaileyEntry | L::CsPrisonEntry | L::CsLibraryEntry | L::CsTheatreEntryNearPrison | L::CsKeepEntryMain | L::CsKeepEntryRamp |L::CSOldSoftlockRoom | L::CsKeepClimbEntrance | L::CsMain | L::CsTheatreEntrance => "ZONE_LowerCastle",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Zone_Library",
            L::SkTheatreEntry | L::SkUnderbellyEntry | L::SkCastleClimbEntry | L::SkCastleMainEntry | L::SkCastleRampEntry| L::SansaKeep | L::Sunsetter => "Zone_Upper",
            L::EbEntryCastle | L::EbEntryRuins | L::EbEntryTheatre| L::EbEntryUnderBelly | L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuinsEntrance | L::TowerRuinsKeep  => "Zone_Tower",
            L::SansaHole |L::PrisonHole| L::BaileyHole| L::MainUnderbelly => "Zone_Caves",
            L::ThBaileyEntry | L::ThCastleEntryMain | L::ThCastleEntryPillar | L::ThKeepEntry | L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Zone_Theatre"
            }
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::PEntryTheatre | L::PEntryCastle| L::PEntryUnderBelly | L::LatePrison | L::VDreamBreaker | L::EarlyPrison| L::StrongEyes=> "Dilapidated Dungeon",
            L::CsBaileyEntry | L::CsPrisonEntry | L::CsLibraryEntry | L::CsTheatreEntryNearPrison | L::CsKeepEntryMain | L::CsKeepEntryRamp |L::CSOldSoftlockRoom | L::CsKeepClimbEntrance | L::CsMain | L::CsTheatreEntrance => "Castle Sansa",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Listless Library",
            L::SkTheatreEntry | L::SkUnderbellyEntry | L::SkCastleClimbEntry | L::SkCastleMainEntry | L::SkCastleRampEntry| L::SansaKeep | L::Sunsetter => "Sansa Keep",
            L::EbEntryCastle | L::EbEntryRuins | L::EbEntryTheatre| L::EbEntryUnderBelly | L::EmptyBailey => "Empty Bailey",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Tower Ruins",
            L::SansaHole | L::PrisonHole |L::BaileyHole| L::MainUnderbelly => "Underbelly",
            L::ThBaileyEntry | L::ThCastleEntryMain | L::ThCastleEntryPillar | L::ThKeepEntry | L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Twilight Theatre"
            }
            L::FinalBoss => "Princess",
        }
    }
}
