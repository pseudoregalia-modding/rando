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
use Lock::{All, Any};
impl Location {
    // need to include some reverse directions
    pub const fn locks(&self) -> Lock {
        match self {
            // Prison / Dilapidated Dungeon
            L::LatePrison => All(&[
                Any(&[
                    A::DreamBreaker.into(),
                    L::CsMain.into(),
                ]),

                Any(&[
                    L::PEntryUnderBelly.into(),
                    L::EarlyPrison.into()
                ]),
            ]),
            L::EarlyPrison => All(&[
                Any(&[
                    A::DreamBreaker.into(),
                    L::CsMain.into(),
                ]),
                Any(&[
                    All(&[
                        L::StrongEyes.into(),
                        Lock::SmallKey
                    ]),
                    L::VDreamBreaker.into(),
                    L::LatePrison.into()
                ]),
            ]),
            L::PEntryUnderBelly => All(&[
                A::DreamBreaker.into(),
                Any(&[
                    L::LatePrison.into(),
                    All(&[
                        L::PrisonHole.into(),
                        A::AscendantLight.into(),
                    ]),
                ]),
            ]),
            L::VDreamBreaker => All(&[
                L::EarlyPrison.into()
            ]),
            L::StrongEyes => All(&[
                Any(&[
                    All(&[
                        L::LatePrison.into(),
                        A::Slide.into()
                    ]),
                    All(&[
                        L::CsMain.into(),
                        Lock::SmallKey,
                        A::DreamBreaker.into(),
                    ]),
                ]),
            ]),
            L::PEntryCastle => All(&[
                Any(&[
                    L::CsPrisonEntry.into(),
                    All(&[
                        L::StrongEyes.into(),
                        Lock::SmallKey,
                        A::DreamBreaker.into()
                    ]),
                ]),
            ]),
            L::PEntryTheatre => All(&[
                Any(&[
                    L::ThDungeonEntry.into(),
                    All(&[
                        L::LatePrison.into(),
                        Any(&[
                            A::ClingGem(6).into(),
                            A::SunGreaves.into(),
                            A::AscendantLight.into(),
                        ]),
                    ]),
                ]),
            ]),
            // Castle Sansa
            L::CsPrisonEntry => All(&[
                Any(&[
                    L::CsMain.into(),
                    L::PEntryCastle.into(),
                ]),
            ]),
            L::CsLibraryEntry => All(&[
                Any(&[
                    L::MainLibrary.into(),
                    All(&[
                        L::CsMain.into(),
                        A::DreamBreaker.into(),
                    ]),
                ]),
            ]),
            L::CsTheatreEntryNearPrison => All(&[
                Any(&[
                    L::PillarRoom.into(),
                    All(&[
                        L::CsMain.into(),
                        Any(&[
                            A::SunGreaves.into(),
                            A::Sunsetter.into(),
                            A::ClingGem(4).into(),
                            All(&[
                                A::Slide.into(),
                                A::SolarWind.into(),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
            L::CsOldSoftlockRoom => All(&[
                
                Any(&[
                    All(&[
                        L::CsMain.into(),
                        A::ClingGem(2).into(),
                    ]),
                    All(&[
                        L::CsTheatreEntrance.into(),
                        A::Slide.into(),
                        Any(&[
                            A::HeliacalPower.into(),
                            A::SunGreaves.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
            ]),
            L::CsKeepClimbEntrance => All(&[
                L::CsMain.into(),
                Lock::SmallKey,
            ]),
            L::CsKeepEntryMain => Any(&[
                L::CsMain.into(),
                L::SansaKeep.into(),
            ]),
            L::CsKeepEntryRamp => Any(&[
                All(&[
                    L::CsMain.into(),
                    Any(&[
                        A::DreamBreaker.into(),
                        A::ClingGem(4).into(),
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                    ]),
                ]),
                L::SansaKeep.into(),
            ]),
            L::CsBaileyEntry => Any(&[
                L::CsMain.into(),
                L::EbEntryCastle.into(),
            ]),
            L::CsMain => All(&[
                Any(&[
                    L::CsPrisonEntry.into(),
                    L::CsBaileyEntry.into(),
                    L::CsTheatreEntryNearPrison.into(),
                    All(&[
                        L::CsOldSoftlockRoom.into(),
                        A::ClingGem(4).into()
                    ]),
                    All(&[
                        A::DreamBreaker.into(),
                        Any(&[
                            L::CsLibraryEntry.into(),
                            All(&[
                                L::CsKeepClimbEntrance.into(),
                                Lock::SmallKey,
                            ]),
                        ]),
                    ]),
                ]),
            ]),
            L::CsTheatreEntrance => Any(&[
                L::ThCastleEntryMain.into(),
                All(&[
                    L::CsOldSoftlockRoom.into(),
                    A::Sunsetter.into(),
                    A::ClingGem(4).into(),
                    Any(&[
                        A::HeliacalPower.into(),
                        A::SunGreaves.into(),
                    ]),
                ]),
            ]),
            // Library
            L::LibSaveNearGreaves => All(&[
                L::MainLibrary.into(), // Can only reach here from main library OR random spawn
                // Enter from the front entrance through the slide slot.
                All(&[
                    A::DreamBreaker.into(),
                    A::Slide.into(),
                    Any(&[
                        A::SolarWind.into(),
                        A::Sunsetter.into(),
                        A::SunGreaves.into(),
                    ]), 
                ]),
                //Enter through Reverse route.
                Any(&[
                    A::SunGreaves.into(), // Add trick MOVEMENT here once implemented, level Advanced.
                    All(&[
                        A::ClingGem(2).into(),
                        Any(&[
                            A::Sunsetter.into(),
                            A::HeliacalPower.into(),
                        ])
                    ]), // Add Movement trick here level Expert
                    All(&[
                        A::Slide.into(),
                        A::SolarWind.into(),
                        A::HeliacalPower.into()
                    ]), // Add Movement trick here level Advanced
                ]),
            ]),
            L::MainLibrary => Any(&[
                All(&[
                    L::CsMain.into(),
                    A::DreamBreaker.into(),
                ]),
                All(&[
                    L::LibSaveNearGreaves.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        All(&[
                            A::HeliacalPower.into(),
                            Any(&[
                                A::ClingGem(2).into(),
                                A::DreamBreaker.into(),
                                All(&[
                                    A::Slide.into(),
                                    A::SolarWind.into(),
                                ]),
                            ]),
                        ]),
                        All(&[
                            A::ClingGem(2).into(),
                            A::Sunsetter.into(),
                        ]),
                    ]),
                ]),
            ]),
            L::Restricted => All(&[
                L::MainLibrary.into(),
                Lock::SmallKey,
                A::DreamBreaker.into(),
            ]),
            // Sansa Keep
            L::SkCastleClimbEntry => All(&[
                L::CsKeepClimbEntrance.into(),
            ]),
            L::SkCastleMainEntry => Any(&[
                L::SansaKeep.into(),
                L::CsKeepEntryMain.into(),
            ]),
            L::SkCastleRampEntry => Any(&[
                All(&[
                    L::SansaKeep.into(),
                    Any(&[
                        A::ClingGem(2).into(),
                        A::SunGreaves.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
                L::CsKeepEntryRamp.into(),
            ]),
            L::SkUnderbellyEntry => Any(&[
                All(&[
                    L::SansaKeep.into(),
                    Any(&[
                        A::Sunsetter.into(),
                        A::HeliacalPower.into(),
                        A::SunGreaves.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
                L::SansaHole.into(),
            ]),
            L::SkTheatreEntry => Any(&[
                L::ThKeepEntry.into(),
                All(&[
                    L::SansaKeep.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
            ]),
            L::SansaKeep => Any(&[
                L::SkCastleRampEntry.into(),
                L::SkCastleMainEntry.into(),
                All(&[
                    L::MainTheatre.into(),
                    A::ClingGem(2).into(),
                ]),
                All(&[
                    L::SkUnderbellyEntry.into(),
                    Any(&[
                        A::Sunsetter.into(),
                        A::HeliacalPower.into(),
                        A::SunGreaves.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                        // Add just MOVMENT trick Normal here since can just backflip up each of em lol.
                    ])
                ]),
            ]),
            L::Sunsetter => All(&[
                L::SansaKeep.into(),
                Any(&[
                    All(&[
                        Lock::SmallKey,
                        A::DreamBreaker.into()
                    ]),
                    A::SunGreaves.into(),
                    A::ClingGem(2).into(),
                ]),
            ]),
            // Bailey
            L::EbEntryCastle => Any(&[
                L::CsBaileyEntry.into(),
                L::EmptyBailey.into(),
            ]),
            L::EbEntryRuins => All(&[
                Any(&[
                    L::TowerRuinsEntrance.into(),
                    L::EmptyBailey.into(),
                ]),
                Any(&[
                    A::SunGreaves.into(),
                    A::Sunsetter.into(),
                    All(&[
                        A::ClingGem(2).into(),
                        A::HeliacalPower.into(),
                    ]),
                    All(&[
                        A::Slide.into(),
                        A::SolarWind.into(),
                    ]),
                ]),
            ]),
            L::EbEntryTheatre => Any(&[
                L::EmptyBailey.into(),
                L::PillarRoom.into(),
            ]),
            L::EbEntryUnderBelly => Any(&[
                L::BaileyHole.into(),
                All(&[
                    L::EmptyBailey.into(),
                    Any(&[
                        A::Sunsetter.into(),
                        A::SunGreaves.into(),
                        A::HeliacalPower.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
            ]),
            L::EmptyBailey => Any(&[
                L::EbEntryCastle.into(),
                L::EbEntryUnderBelly.into(),
                L::EbEntryTheatre.into(),
                All(&[
                    L::EbEntryRuins.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                        All(&[
                            A::ClingGem(2).into(),
                            A::HeliacalPower.into(),
                        ]),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
            ]),
            // Tower
            L::TowerRuinsEntrance => Any(&[
                All(&[
                    L::EmptyBailey.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                        All(&[
                            A::ClingGem(2).into(),
                            A::HeliacalPower.into(),
                        ]),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                        ]),
                    ]),
                ]),
                L::TowerRuinsKeep.into(),
            ]),
            L::TowerRuinsKeep => Any(&[
                All(&[
                    L::TowerRuinsEntrance.into(),
                    Any(&[
                        //ADD MOVEMENT TRICK AND CLING ABUSE HERE. EXPERT / ADVANCED 
                        A::SunGreaves.into(),
                        All(&[
                            A::ClingGem(2).into(),
                            Any(&[
                                A::Sunsetter.into(),
                                A::HeliacalPower.into(),
                            ]),
                        ]),
                    ]),
                ]),
                L::FinalBoss.into()
            ]),
            // Underbelly
            L::PrisonHole => Any(&[
                All(&[
                    L::PEntryUnderBelly.into(),
                    A::DreamBreaker.into(),
                ]),
                All(&[
                    L::MainUnderbelly.into(), // From main to the hole (right below the gear mobs.)
                    Any(&[
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into()
                        ]),
                    ]),
                ]),
            ]),
            L::BaileyHole => Any(&[
                All(&[
                    A::Sunsetter.into(),
                    Any(&[
                        L::EbEntryUnderBelly.into(),
                        L::SansaHole.into(),
                    ]),
                ]),
                L::MainUnderbelly.into(), // From main to hole.
            ]),
            L::SansaHole => Any(&[
                All(&[
                    A::Sunsetter.into(),
                    Any(&[
                        L::MainUnderbelly.into(),
                        L::BaileyHole.into(),
                        All(&[
                            L::HpSave.into(),
                            A::Slide.into(),
                        ]),
                    ]),
                ]),
                All(&[
                    L::SkUnderbellyEntry.into(),
                    Any(&[
                        A::HeliacalPower.into(),
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                    ]),
                ]),
            ]),
            L::MainUnderbelly => Any(&[
                L::PrisonHole.into(),
                All(&[
                    L::BaileyHole.into(),
                    A::SunGreaves.into(),
                    A::HeliacalPower.into(), 
                    A::Sunsetter.into(),
                ]), // First bubble directly to circular platforms.
                All(&[
                    L::HpSave.into(),
                    Any(&[
                        A::DreamBreaker.into(),
                        // Below is sliding through the gap above the hanging block and then doing an ultra to skip the lever.
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                            A::SunGreaves.into(),
                            A::ClingGem(2).into()
                        ]),
                    ]),
                ]),
                All(&[
                    L::SansaHole.into(),
                    A::Sunsetter.into(), 
                    A::Slide.into(),
                ]), // from Sansa hole (above going to Major Key)
            ]),
            L::VAscendantLight => All(&[
                L::PrisonHole.into(),
                A::DreamBreaker.into(),
            ]),
            L::HpSave => Any(&[
                All(&[
                    L::BaileyHole.into(),
                    A::Slide.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        A::Sunsetter.into(),
                        A::HeliacalPower.into(),
                    ]),
                ]),
                All(&[
                    L::MainUnderbelly.into(),
                    A::DreamBreaker.into(),
                    Any(&[
                        All(&[
                            A::HeliacalPower.into(),
                            Any(&[
                                A::SunGreaves.into(),
                                All(&[
                                    A::Slide.into(),
                                    A::SolarWind.into(),
                                ]),
                            ]),
                        ]),
                        All(&[
                            A::ClingGem(2).into(),
                            A::Sunsetter.into(),
                        ])
                    ]),
                ]),
            ]),
            //Theatre
            L::ThBaileyEntry => Any(&[
                L::EbEntryTheatre.into(),
                All(&[
                    L::PillarRoom.into(), 
                    Any(&[
                        A::SunGreaves.into(),
                        A::HeliacalPower.into(),
                        A::ClingGem(2).into(),
                        A::Sunsetter.into(),
                    ]),
                ]),
            ]),
            L::ThCastleEntryMain => Any(&[
                L::CsTheatreEntrance.into(),
                L::TheatreEntrance.into(),
            ]),
            L::ThCastleEntryPillar => Any(&[
                L::CsTheatreEntryNearPrison.into(),
                All(&[
                    L::PillarRoom.into(), 
                    Any(&[
                        A::SunGreaves.into(),
                        A::HeliacalPower.into(),
                        A::ClingGem(2).into(),
                        A::Sunsetter.into(),
                    ]),
                ]),
            ]),
            L::ThKeepEntry => Any(&[
                L::SkTheatreEntry.into(),
                All(&[
                    L::OtherTheatrePath.into(), 
                    Any(&[
                        All(&[A::AscendantLight.into(), A::DreamBreaker.into()]), 
                        A::HeliacalPower.into(),
                        A::ClingGem(2).into(),
                    ]),
                ]),
            ]),
            L::ThDungeonEntry => Any(&[
                All(&[
                    L::OtherTheatrePath.into(),
                    Any(&[
                        All(&[A::AscendantLight.into(), A::DreamBreaker.into()]),
                        All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
                        A::SunGreaves.into(),
                        A::ClingGem(2).into(),
                    ]),
                ]),
                L::PEntryTheatre.into(),
            ]),
            L::OtherTheatrePath => Any(&[
                All(&[
                    Any(&[L::ThKeepEntry.into(), L::ThDungeonEntry.into()]),
                    Any(&[
                        A::ClingGem(2).into(),
                        All(&[
                            A::AscendantLight.into(),
                            A::DreamBreaker.into(),
                        ]),
                    ]),
                ]),
                All(&[L::ThKeepEntry.into(), A::HeliacalPower.into()]),
                All(&[
                    L::ThDungeonEntry.into(),
                    Any(&[
                        A::SunGreaves.into(),
                        All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
                    ]),
                ]),
            ]),
            L::PillarRoom => All(&[
                Any(&[L::ThCastleEntryPillar.into(), L::ThBaileyEntry.into()]),
                Any(&[
                    A::SunGreaves.into(),
                    A::HeliacalPower.into(),
                    A::ClingGem(2).into(),
                    A::Sunsetter.into(),
                ]),
            ]),
            L::TheatreEntrance => Any(&[
                L::MainTheatre.into(),
                All(&[
                    L::ThCastleEntryMain.into(),
                    Any(&[
                        A::ClingGem(2).into(),
                        All(&[
                            A::Slide.into(),
                            A::SolarWind.into(),
                            A::SunGreaves.into(),
                        ]),
                        All(&[
                            A::HeliacalPower.into(),
                            A::Sunsetter.into(),
                        ]),
                    ]),
                ]),
            ]),
            L::MainTheatre => Any(&[
                All(&[
                    L::TheatreEntrance.into(),
                    Any(&[
                        A::ClingGem(2).into(),
                        A::SunGreaves.into(),
                        All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
                    ]),
                ]),
                All(&[
                    L::PillarRoom.into(),
                    A::Sunsetter.into(),
                    Any(&[
                        A::ClingGem(2).into(),
                        All(&[A::SunGreaves.into(), A::HeliacalPower.into()]),
                    ]),
                ]),
                All(&[
                    L::OtherTheatrePath.into(),
                    A::ClingGem(2).into(),
                    Any(&[
                        A::SunGreaves.into(),
                        A::HeliacalPower.into(),
                        All(&[A::Slide.into(), A::SolarWind.into()]),
                    ]),
                ]),
            ]),
            // Final Boss
            L::FinalBoss => Any(&[
                All(&[
                    L::TowerRuinsEntrance.into(),
                    A::ClingGem(2).into(),
                    Any(&[
                        A::SunGreaves.into(),
                        All(&[
                            A::HeliacalPower.into(),
                            A::Sunsetter.into(),
                        ]),
                    ]),
                ]),
                Lock::Ending,
            ]),
        }
    }
    pub const fn file(&self) -> &'static str {
        match self {
            L::PEntryTheatre | L::PEntryCastle| L::PEntryUnderBelly | L::LatePrison | L::VDreamBreaker | L::EarlyPrison| L::StrongEyes => "ZONE_Dungeon",
            L::CsBaileyEntry | L::CsPrisonEntry | L::CsLibraryEntry | L::CsTheatreEntryNearPrison | L::CsKeepEntryMain | L::CsKeepEntryRamp |L::CsOldSoftlockRoom | L::CsKeepClimbEntrance | L::CsMain | L::CsTheatreEntrance => "ZONE_LowerCastle",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Zone_Library",
            L::SkTheatreEntry | L::SkUnderbellyEntry | L::SkCastleClimbEntry | L::SkCastleMainEntry | L::SkCastleRampEntry| L::SansaKeep | L::Sunsetter => "Zone_Upper",
            L::EbEntryCastle | L::EbEntryRuins | L::EbEntryTheatre| L::EbEntryUnderBelly | L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuinsEntrance | L::TowerRuinsKeep  => "Zone_Tower",
            L::VAscendantLight | L::HpSave | L::SansaHole |L::PrisonHole| L::BaileyHole| L::MainUnderbelly => "Zone_Caves",
            L::ThDungeonEntry | L::ThBaileyEntry | L::ThCastleEntryMain | L::ThCastleEntryPillar | L::ThKeepEntry | L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Zone_Theatre"
            }
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::PEntryTheatre | L::PEntryCastle| L::PEntryUnderBelly | L::LatePrison | L::VDreamBreaker | L::EarlyPrison| L::StrongEyes=> "Dilapidated Dungeon",
            L::CsBaileyEntry | L::CsPrisonEntry | L::CsLibraryEntry | L::CsTheatreEntryNearPrison | L::CsKeepEntryMain | L::CsKeepEntryRamp |L::CsOldSoftlockRoom | L::CsKeepClimbEntrance | L::CsMain | L::CsTheatreEntrance => "Castle Sansa",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Listless Library",
            L::SkTheatreEntry | L::SkUnderbellyEntry | L::SkCastleClimbEntry | L::SkCastleMainEntry | L::SkCastleRampEntry| L::SansaKeep | L::Sunsetter => "Sansa Keep",
            L::EbEntryCastle | L::EbEntryRuins | L::EbEntryTheatre| L::EbEntryUnderBelly | L::EmptyBailey => "Empty Bailey",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Tower Ruins",
            L::VAscendantLight | L::HpSave | L::SansaHole | L::PrisonHole |L::BaileyHole| L::MainUnderbelly => "Underbelly",
            L::ThDungeonEntry | L::ThBaileyEntry | L::ThCastleEntryMain | L::ThCastleEntryPillar | L::ThKeepEntry | L::PillarRoom | L::TheatreEntrance | L::OtherTheatrePath | L::MainTheatre => {
                "Twilight Theatre"
            }
            L::FinalBoss => "Princess",
        }
    }
}
