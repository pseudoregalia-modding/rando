use super::*;
/*
To-Do:
split areas further to ensure proper logic is available.
Add Nodes for each lever.
Re-name current nodes.
i.e. split Late prison into the save crystal's, In the rafters room, and after the underbelly wall.

*/
#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount, strum::Display)]
pub enum Location {
    // Prison
    VDreamBreaker,
    EarlyPrison,
    LatePrison,
    StrongEyes,
    PEntryCastle,
    PEntryUnderBelly,
    PEntryTheatre,
    // Castle
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
    // Library
    MainLibrary,
    Restricted,
    LibSaveNearGreaves,
    // Keep
    SkCastleRampEntry,
    SkCastleMainEntry,
    SkCastleClimbEntry,
    SkUnderbellyEntry,
    SkTheatreEntry,
    SansaKeep,
    Sunsetter,
    // Bailey
    EmptyBailey,
    EbEntryUnderBelly,
    EbEntryRuins,
    EbEntryTheatre,
    EbEntryCastle,
    // Ruins
    TowerRuinsEntrance,
    TowerRuinsKeep,
    // Underbelly
    SansaHole,
    BaileyHole,
    PrisonHole,
    MainUnderbelly,
    VAscendantLight,
    HpSave,
    // Theatre
    ThCastleEntryPillar,
    ThCastleEntryMain,
    ThBaileyEntry,
    ThKeepEntry,
    ThDungeonEntry,
    PillarRoom,
    TheatreEntrance,
    OtherTheatrePath,
    MainTheatre,
    // Final
    FinalBoss,
}

use Ability as A;
use Location as L;
use Lock::{All, Any, Location as Loc, Movement as Powerup, Trick};
use tricks::Trick as T;
use Difficulty as D;
impl Location {
    // need to include some reverse directions
    pub const fn locks(&self) -> Lock {
        match self {
            // Prison / Dilapidated Dungeon
            L::LatePrison => All(&[
                Powerup(A::DreamBreaker),
                Any(&[
                    Loc(L::CsMain),
                    Loc(L::PEntryUnderBelly),
                    Loc(L::EarlyPrison),
                ]),
            ]),
            L::EarlyPrison => Any(&[
                Loc(L::CsMain), // Drop in from castle
                All(&[
                    Powerup(A::DreamBreaker),
                    Any(&[
                        All(&[Loc(L::StrongEyes), Lock::SmallKey]), // From Strong Eyes
                        Loc(L::VDreamBreaker), // Breaking wall from DB item check
                        Loc(L::LatePrison),    // Breaking wall from late.
                    ]),
                ]),
                All(&[
                    Loc(L::VDreamBreaker),
                    Any(&[
                        All(&[
                            Powerup(A::SunGreaves),
                            Trick(T::Movement, D::Expert),
                            Trick(T::OneWall, D::Advanced),
                        ]),
                        All(&[
                            Powerup(A::SunGreaves),
                            Powerup(A::SolarWind),
                            Trick(T::Movement, D::Advanced),
                            Trick(T::OneWall, D::Advanced),
                        ]),
                    ]),
                ])
            ]),
            L::PEntryUnderBelly => Any(&[
                All(&[Loc(L::LatePrison), Powerup(A::DreamBreaker)]),
                All(&[Loc(L::PrisonHole), Powerup(A::AscendantLight)]),
            ]),
            L::VDreamBreaker => Loc(L::EarlyPrison),
            L::StrongEyes => Any(&[
                All(&[Loc(L::LatePrison), Powerup(A::Slide)]),
                All(&[Loc(L::CsMain), Lock::SmallKey, Powerup(A::DreamBreaker)]),
            ]),
            L::PEntryCastle => Any(&[
                Loc(L::CsPrisonEntry),
                All(&[Loc(L::StrongEyes), Lock::SmallKey, Powerup(A::DreamBreaker)]),
                All(&[
                    Loc(L::EarlyPrison),
                    Powerup(A::SolarWind),
                    Powerup(A::SunGreaves),
                    Powerup(A::HeliacalPower),
                    Powerup(A::Sunsetter),
                    Powerup(A::ClingGem(6)),
                    Trick(T::ClingAbuse, D::Expert),
                    Trick(T::Movement, D::Expert),
                    Trick(T::Momentum, D::Advanced)

                ]),
            ]),
            L::PEntryTheatre => Any(&[
                Loc(L::ThDungeonEntry),
                All(&[
                    Loc(L::LatePrison),
                    Any(&[
                        Powerup(A::ClingGem(6)),
                        Powerup(A::SunGreaves),
                        Powerup(A::AscendantLight),
                    ]),
                ]),
            ]),
            // Castle Sansa
            L::CsPrisonEntry => Any(&[Loc(L::CsMain), Loc(L::PEntryCastle)]),
            L::CsLibraryEntry => Any(&[
                Loc(L::MainLibrary),
                All(&[Loc(L::CsMain), Powerup(A::DreamBreaker)]),
            ]),
            L::CsTheatreEntryNearPrison => Any(&[
                Loc(L::PillarRoom),
                All(&[
                    Loc(L::CsMain),
                    Any(&[
                        All(&[
                            Powerup(A::Slide),
                            Trick(T::Movement, D::Advanced),
                            Trick(T::Momentum, D::Advanced),
                        ]),
                        All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                        All(&[Powerup(A::HeliacalPower), Powerup(A::SolarWind)]), // This is using the little block stair case thing to get up
                        All(&[Powerup(A::SunGreaves), Trick(T::Movement, D::Normal)]),
                        All(&[Powerup(A::Sunsetter), Trick(T::Movement, D::Advanced)]),
                        All(&[Powerup(A::ClingGem(4)), Trick(T::ClingAbuse, D::Normal)]),
                        All(&[
                            Powerup(A::Slide),
                            Powerup(A::SolarWind),
                            Trick(T::Movement, D::Normal),
                        ]),
                    ]),
                ]),
            ]),
            L::CsOldSoftlockRoom => Any(&[
                All(&[Loc(L::CsMain), Powerup(A::ClingGem(2))]),
                All(&[
                    Loc(L::CsTheatreEntrance),
                    Any(&[
                        All(&[
                            Powerup(A::SolarWind),
                            Powerup(A::HeliacalPower),
                            Trick(T::Movement, D::Normal),
                        ]),
                        All(&[
                            Powerup(A::Slide),
                            Powerup(A::ClingGem(2)),
                            Trick(T::Movement, D::Advanced),
                        ]),
                    ]),
                ]),
            ]),
            L::CsKeepClimbEntrance => All(&[Loc(L::CsMain), Lock::SmallKey]),
            L::CsKeepEntryMain => Any(&[Loc(L::CsMain), Loc(L::SansaKeep)]),
            L::CsKeepEntryRamp => Any(&[
                All(&[
                    Loc(L::CsMain),
                    Any(&[
                        Powerup(A::DreamBreaker),
                        Trick(T::Movement, D::Normal),
                        Powerup(A::SunGreaves),
                        Powerup(A::Sunsetter),
                    ]),
                ]),
                Loc(L::SansaKeep),
            ]),
            L::CsBaileyEntry => Any(&[Loc(L::CsMain), Loc(L::EbEntryCastle)]),
            L::CsMain => Any(&[
                Loc(L::CsPrisonEntry),
                Loc(L::CsBaileyEntry),
                Loc(L::CsTheatreEntryNearPrison),
                All(&[Loc(L::CsOldSoftlockRoom), 
                    Any(&[
                        Powerup(A::ClingGem(4)),
                        All(&[Powerup(A::SunGreaves), Trick(T::Movement, D::Expert), Trick(T::OneWall, D::Advanced)]),
                        All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced), Trick(T::OneWall, D::Advanced)]),
                    ]),
                ]),
                All(&[
                    Powerup(A::DreamBreaker),
                    Any(&[
                        Loc(L::CsLibraryEntry),
                        All(&[Loc(L::CsKeepClimbEntrance), Lock::SmallKey]),
                    ]),
                ]),
            ]),
            L::CsTheatreEntrance => Any(&[
                Loc(L::ThCastleEntryMain),
                All(&[
                    Loc(L::CsOldSoftlockRoom),
                    Powerup(A::Sunsetter),
                    Powerup(A::ClingGem(4)),
                    Powerup(A::HeliacalPower),
                ]),
                All(&[
                    Powerup(A::SolarWind),
                    Powerup(A::SunGreaves),
                    Trick(T::Movement, D::Advanced),
                ]),
                All(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SunGreaves),
                    Powerup(A::Sunsetter),
                    Trick(T::OneWall, D::Advanced),
                    Trick(T::Movement, D::Advanced),
                ])
            ]),
            // Library
            L::LibSaveNearGreaves => All(&[
                Loc(L::MainLibrary), // Can only reach here from main library OR random spawn
                Any(&[
                    // Enter from the front entrance through the slide slot.
                    All(&[
                        Powerup(A::DreamBreaker),
                        Powerup(A::Slide),
                    ]),
                    // Enter through Reverse route.
                    All(&[Powerup(A::SunGreaves), Trick(T::Movement, D::Advanced)]), 
                    All(&[
                        Powerup(A::ClingGem(2)),
                        Any(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
                        Trick(T::Movement, D::Expert)
                    ]), 
                    All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]), 
                ]),
            ]),
            L::MainLibrary => Any(&[
                All(&[Loc(L::CsMain), Powerup(A::DreamBreaker)]),
                All(&[
                    Loc(L::LibSaveNearGreaves),
                    Any(&[
                        Powerup(A::SunGreaves),    
                        Powerup(A::ClingGem(2)),
                        All(&[Powerup(A::DreamBreaker), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
                        All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower), Trick(T::Movement, D::Normal)]),
                    ]),
                ]),
            ]),
            L::Restricted => All(&[
                Loc(L::MainLibrary),
                Lock::SmallKey,
                Powerup(A::DreamBreaker),
            ]),
            // Sansa Keep
            L::SkCastleClimbEntry => Loc(L::CsKeepClimbEntrance),
            L::SkCastleMainEntry => Any(&[Loc(L::SansaKeep), Loc(L::CsKeepEntryMain)]),
            L::SkCastleRampEntry => Any(&[
                All(&[
                    Loc(L::SansaKeep),
                    Any(&[
                        Powerup(A::ClingGem(2)),
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Normal), Trick(T::OneWall, D::Normal)]),
                        Powerup(A::Slide),
                    ]),
                ]),
                Loc(L::CsKeepEntryRamp),
            ]),
            L::SkUnderbellyEntry => Any(&[
                All(&[
                    Loc(L::SansaKeep),
                    Any(&[
                        Powerup(A::Sunsetter),
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind),
                    ]),
                ]),
                Loc(L::SansaHole),
            ]),
            L::SkTheatreEntry => Any(&[
                Loc(L::ThKeepEntry),
                All(&[
                    Loc(L::SansaKeep),
                    Any(&[
                        Powerup(A::SunGreaves),
                        Powerup(A::SolarWind),
                        Powerup(A::ClingGem(2)),
                        All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
                    ]),
                ]),
            ]),
            L::SansaKeep => Any(&[
                Loc(L::SkCastleRampEntry),
                Loc(L::SkCastleMainEntry),
                All(&[
                    Loc(L::SkTheatreEntry), 
                    Any(&[
                        Powerup(A::ClingGem(2)),
                        Powerup(A::SolarWind),
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
                    ]),
                ]),
                All(&[
                    Loc(L::SkUnderbellyEntry),
                    Any(&[
                        Powerup(A::Sunsetter),
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind),
                        Trick(T::Movement, D::Normal)
                    ]),
                ]),
            ]),
            L::Sunsetter => All(&[
                Loc(L::SansaKeep),
                Any(&[
                    All(&[Lock::SmallKey, Powerup(A::DreamBreaker)]),
                    Powerup(A::SunGreaves),
                    Powerup(A::ClingGem(2)),
                    Powerup(A::Sunsetter),
                    Trick(T::Movement, D::Advanced),
                ]),
            ]),
            // Bailey
            L::EbEntryCastle => Any(&[Loc(L::CsBaileyEntry), Loc(L::EmptyBailey)]),
            L::EbEntryRuins => All(&[
                Any(&[Loc(L::TowerRuinsEntrance), Loc(L::EmptyBailey)]),
                Any(&[
                    Powerup(A::SunGreaves),
                    All(&[Powerup(A::Sunsetter), Trick(T::SunsetterAbuse, D::Advanced)]),
                    All(&[Powerup(A::ClingGem(2)), Powerup(A::HeliacalPower)]),
                    Powerup(A::SolarWind),
                ]),
            ]),
            L::EbEntryTheatre => Any(&[Loc(L::EmptyBailey), Loc(L::PillarRoom)]),
            L::EbEntryUnderBelly => Any(&[
                Loc(L::BaileyHole),
                All(&[
                    Loc(L::EmptyBailey),
                    Any(&[
                        Powerup(A::Sunsetter),
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind),
                    ]),
                ]),
            ]),
            L::EmptyBailey => Any(&[
                Loc(L::EbEntryCastle),
                Loc(L::EbEntryUnderBelly),
                Loc(L::EbEntryTheatre),
                All(&[
                    Loc(L::EbEntryRuins),
                    Any(&[
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::Sunsetter), Trick(T::SunsetterAbuse, D::Advanced)]),
                        All(&[Powerup(A::ClingGem(2)), Powerup(A::HeliacalPower)]),
                        Powerup(A::SolarWind),
                    ]),
                ]),
            ]),
            // Tower
            L::TowerRuinsEntrance => Any(&[
                All(&[
                    Loc(L::EbEntryRuins),
                    Any(&[
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::Sunsetter), Trick(T::SunsetterAbuse, D::Advanced)]),
                        All(&[Powerup(A::ClingGem(2)), Powerup(A::HeliacalPower)]),
                        Powerup(A::SolarWind),
                    ]),
                ]),
                Loc(L::TowerRuinsKeep),
            ]),
            L::TowerRuinsKeep => Any(&[
                All(&[
                    Loc(L::TowerRuinsEntrance),
                    Any(&[
                        All(&[Powerup(A::SunGreaves), Trick(T::Movement, D::Expert)]),
                        All(&[Powerup(A::SolarWind), Powerup(A::AscendantLight), Trick(T::Movement, D::Normal)]),
                        //All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower), Trick(T::Movement, D::Expert)]), // needs 2 kicks but Sungreaves already has its own
                        All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower), Trick(T::Movement, D::Expert), Trick(T::ClingAbuse, D::Advanced)]),
                        All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::Sunsetter)]), // Intended route
                    ]),
                ]),
                Loc(L::FinalBoss),
            ]),
            // Underbelly
            L::PrisonHole => Any(&[
                All(&[Loc(L::PEntryUnderBelly), Powerup(A::DreamBreaker)]),
                All(&[
                    Loc(L::MainUnderbelly), // From main to the hole (right below the gear mobs.)
                    Any(&[
                        Powerup(A::SunGreaves),
                        Powerup(A::Sunsetter),
                        Powerup(A::SolarWind),
                    ]),
                ]),
            ]),
            L::BaileyHole => Any(&[
                All(&[
                    Powerup(A::Sunsetter),
                    Any(&[Loc(L::EbEntryUnderBelly), Loc(L::SansaHole)]),
                ]),
                Loc(L::MainUnderbelly), // From main to hole.
            ]),
            L::SansaHole => Any(&[
                All(&[
                    Powerup(A::Sunsetter),
                    Any(&[
                        Loc(L::MainUnderbelly),
                        Loc(L::BaileyHole),
                        All(&[Loc(L::HpSave), Powerup(A::Slide)]),
                    ]),
                ]),
                Loc(L::SkUnderbellyEntry),
            ]),
            L::MainUnderbelly => Any(&[
                Loc(L::PrisonHole),
                All(&[
                    Loc(L::BaileyHole),
                    Powerup(A::SunGreaves),
                    Powerup(A::HeliacalPower),
                    Powerup(A::Sunsetter),
                    Trick(T::Movement, D::Advanced),
                    Trick(T::OneWall, D::Advanced),
                ]), // First bubble directly to circular platforms.
                All(&[
                    Loc(L::HpSave),
                    Any(&[
                        Powerup(A::DreamBreaker),
                        // Below is sliding through the gap above the hanging block and then doing an ultra to skip the lever.
                        All(&[
                            Powerup(A::SolarWind),
                            Powerup(A::SunGreaves),
                            Powerup(A::ClingGem(2)),
                        ]),
                    ]),
                ]),
                All(&[Loc(L::SansaHole), Powerup(A::Sunsetter), Powerup(A::Slide)]), // from Sansa hole (above going to Major Key)
            ]),
            L::VAscendantLight => All(&[Loc(L::PrisonHole), Powerup(A::DreamBreaker)]),
            L::HpSave => Any(&[
                All(&[
                    Loc(L::BaileyHole),
                    Powerup(A::Slide),
                    Any(&[Powerup(A::Sunsetter), All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Normal)]),]),
                ]),
                All(&[
                    Loc(L::MainUnderbelly),
                    Powerup(A::DreamBreaker),
                    Any(&[
                        All(&[
                            Powerup(A::HeliacalPower),
                            Any(&[Powerup(A::SunGreaves), Powerup(A::SolarWind)]),
                        ]),
                        All(&[Powerup(A::ClingGem(2)), Powerup(A::Sunsetter)]),
                    ]),
                ]),
            ]),
            //Theatre
            L::ThBaileyEntry => Any(&[
                Loc(L::EbEntryTheatre),
                All(&[
                    Loc(L::PillarRoom),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::ClingGem(2)),
                        Powerup(A::Sunsetter),
                    ]),
                ]),
            ]),
            L::ThCastleEntryMain => Any(&[Loc(L::CsTheatreEntrance), Loc(L::TheatreEntrance)]),
            L::ThCastleEntryPillar => Any(&[
                Loc(L::CsTheatreEntryNearPrison),
                All(&[
                    Loc(L::PillarRoom),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::ClingGem(2)),
                        Powerup(A::Sunsetter),
                    ]),
                ]),
            ]),
            L::ThKeepEntry => Any(&[
                Loc(L::SkTheatreEntry),
                All(&[
                    Loc(L::OtherTheatrePath),
                    Any(&[
                        Powerup(A::AscendantLight),
                        Powerup(A::HeliacalPower),
                        Powerup(A::ClingGem(2)),
                    ]),
                ]),
            ]),
            L::ThDungeonEntry => Any(&[
                All(&[
                    Loc(L::OtherTheatrePath),
                    Any(&[
                        Powerup(A::AscendantLight),
                        All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
                        Powerup(A::SunGreaves),
                        Powerup(A::ClingGem(2)),
                    ]),
                ]),
                Loc(L::PEntryTheatre),
            ]),
            L::OtherTheatrePath => Any(&[
                All(&[
                    Any(&[Loc(L::ThKeepEntry), Loc(L::ThDungeonEntry)]),
                    Any(&[Powerup(A::ClingGem(2)), Powerup(A::AscendantLight)]),
                ]),
                All(&[Loc(L::ThKeepEntry), Powerup(A::HeliacalPower)]),
                All(&[
                    Loc(L::ThDungeonEntry),
                    Any(&[
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
                    ]),
                ]),
            ]),
            L::PillarRoom => All(&[
                Any(&[Loc(L::ThCastleEntryPillar), Loc(L::ThBaileyEntry)]),
                Any(&[
                    Powerup(A::HeliacalPower),
                    Powerup(A::ClingGem(2)),
                    Powerup(A::Sunsetter),
                ]),
            ]),
            L::TheatreEntrance => Any(&[
                All(&[
                    Loc(L::MainTheatre),
                    Any(&[
                        Powerup(A::ClingGem(1)),
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind),
                    ]),
                ]),
                All(&[
                    Loc(L::ThCastleEntryMain),
                    Any(&[
                        Powerup(A::ClingGem(2)),
                        All(&[
                            Powerup(A::Slide),
                            Powerup(A::SolarWind),
                            Powerup(A::SunGreaves),
                        ]),
                        All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
                    ]),
                ]),
            ]),
            L::MainTheatre => Any(&[
                All(&[
                    Loc(L::TheatreEntrance),
                    Any(&[
                        Powerup(A::ClingGem(2)),
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
                    ]),
                ]),
                All(&[
                    Loc(L::PillarRoom),
                    Powerup(A::Sunsetter),
                    Any(&[
                        Powerup(A::ClingGem(2)),
                        All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
                    ]),
                ]),
                All(&[
                    Loc(L::OtherTheatrePath),
                    Powerup(A::ClingGem(2)),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        All(&[Powerup(A::Slide), Powerup(A::SolarWind)]),
                    ]),
                ]),
            ]),
            // Final Boss
            L::FinalBoss => Any(&[
                All(&[
                    Loc(L::TowerRuinsEntrance),
                    Powerup(A::ClingGem(2)),
                    Any(&[
                        Powerup(A::SunGreaves),
                        All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
                    ]),
                ]),
                Lock::Ending,
            ]),
        }
    }
    pub const fn file(&self) -> &'static str {
        match self {
            L::PEntryTheatre
            | L::PEntryCastle
            | L::PEntryUnderBelly
            | L::LatePrison
            | L::VDreamBreaker
            | L::EarlyPrison
            | L::StrongEyes => "ZONE_Dungeon",
            L::CsBaileyEntry
            | L::CsPrisonEntry
            | L::CsLibraryEntry
            | L::CsTheatreEntryNearPrison
            | L::CsKeepEntryMain
            | L::CsKeepEntryRamp
            | L::CsOldSoftlockRoom
            | L::CsKeepClimbEntrance
            | L::CsMain
            | L::CsTheatreEntrance => "ZONE_LowerCastle",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Zone_Library",
            L::SkTheatreEntry
            | L::SkUnderbellyEntry
            | L::SkCastleClimbEntry
            | L::SkCastleMainEntry
            | L::SkCastleRampEntry
            | L::SansaKeep
            | L::Sunsetter => "Zone_Upper",
            L::EbEntryCastle
            | L::EbEntryRuins
            | L::EbEntryTheatre
            | L::EbEntryUnderBelly
            | L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Zone_Tower",
            L::VAscendantLight
            | L::HpSave
            | L::SansaHole
            | L::PrisonHole
            | L::BaileyHole
            | L::MainUnderbelly => "Zone_Caves",
            L::ThDungeonEntry
            | L::ThBaileyEntry
            | L::ThCastleEntryMain
            | L::ThCastleEntryPillar
            | L::ThKeepEntry
            | L::PillarRoom
            | L::TheatreEntrance
            | L::OtherTheatrePath
            | L::MainTheatre => "Zone_Theatre",
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::PEntryTheatre
            | L::PEntryCastle
            | L::PEntryUnderBelly
            | L::LatePrison
            | L::VDreamBreaker
            | L::EarlyPrison
            | L::StrongEyes => "Dilapidated Dungeon",
            L::CsBaileyEntry
            | L::CsPrisonEntry
            | L::CsLibraryEntry
            | L::CsTheatreEntryNearPrison
            | L::CsKeepEntryMain
            | L::CsKeepEntryRamp
            | L::CsOldSoftlockRoom
            | L::CsKeepClimbEntrance
            | L::CsMain
            | L::CsTheatreEntrance => "Castle Sansa",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Listless Library",
            L::SkTheatreEntry
            | L::SkUnderbellyEntry
            | L::SkCastleClimbEntry
            | L::SkCastleMainEntry
            | L::SkCastleRampEntry
            | L::SansaKeep
            | L::Sunsetter => "Sansa Keep",
            L::EbEntryCastle
            | L::EbEntryRuins
            | L::EbEntryTheatre
            | L::EbEntryUnderBelly
            | L::EmptyBailey => "Empty Bailey",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Tower Ruins",
            L::VAscendantLight
            | L::HpSave
            | L::SansaHole
            | L::PrisonHole
            | L::BaileyHole
            | L::MainUnderbelly => "Underbelly",
            L::ThDungeonEntry
            | L::ThBaileyEntry
            | L::ThCastleEntryMain
            | L::ThCastleEntryPillar
            | L::ThKeepEntry
            | L::PillarRoom
            | L::TheatreEntrance
            | L::OtherTheatrePath
            | L::MainTheatre => "Twilight Theatre",
            L::FinalBoss => "Princess",
        }
    }
}
