use super::*;

#[derive(PartialEq)]
pub enum Node {
    Location(logic::Location),
    Check(logic::Check),
}

#[derive(Copy, Clone, PartialEq, strum::AsRefStr, strum::EnumIter, strum::Display)]
pub enum Area {
    #[strum(serialize = "Dungeon")]
    Dungeon,
    #[strum(serialize = "Castle Sansa")]
    CastleSansa,
    #[strum(serialize = "Sansa Keep")]
    SansaKeep,
    #[strum(serialize = "Listless Library")]
    ListlessLibrary,
    #[strum(serialize = "Twilight Theatre")]
    TwilightTheatre,
    #[strum(serialize = "Empty Bailey")]
    EmptyBailey,
    #[strum(serialize = "Underbelly")]
    Underbelly,
    #[strum(serialize = "Tower Ruins")]
    TowerRuins,
}

use logic::Location as L;

impl Area {
    pub const fn rooms(&self) -> &[L] {
        match self {
            Area::Dungeon => &[
                L::VDreamBreaker,
                L::EarlyPrison,
                L::LatePrison,
                L::StrongEyes,
                L::PEntryCastle,
                L::PEntryUnderBelly,
                L::PEntryTheatre,
            ],
            Area::CastleSansa => &[
                L::CsOldSoftlockRoom,
                L::CsKeepClimbEntrance,
                L::CsMain,
                L::CsTheatreEntrance,
                L::CsPrisonEntry,
                L::CsLibraryEntry,
                L::CsTheatreEntryNearPrison,
                L::CsKeepEntryMain,
                L::CsKeepEntryRamp,
                L::CsBaileyEntry,
            ],
            Area::SansaKeep => &[
                L::SkCastleRampEntry,
                L::SkCastleMainEntry,
                L::SkCastleClimbEntry,
                L::SkUnderbellyEntry,
                L::SkTheatreEntry,
                L::SansaKeep,
                L::Sunsetter,
            ],
            Area::ListlessLibrary => &[L::MainLibrary, L::Restricted, L::LibSaveNearGreaves],
            Area::TwilightTheatre => &[
                L::ThCastleEntryPillar,
                L::ThCastleEntryMain,
                L::ThBaileyEntry,
                L::ThKeepEntry,
                L::ThDungeonEntry,
                L::PillarRoom,
                L::TheatreEntrance,
                L::OtherTheatrePath,
                L::MainTheatre,
            ],
            Area::EmptyBailey => &[
                L::EmptyBailey,
                L::EbEntryUnderBelly,
                L::EbEntryRuins,
                L::EbEntryTheatre,
                L::EbEntryCastle,
            ],
            Area::Underbelly => &[
                L::SansaHole,
                L::BaileyHole,
                L::PrisonHole,
                L::MainUnderbelly,
                L::VAscendantLight,
                L::HpSave,
            ],
            Area::TowerRuins => &[L::TowerRuinsEntrance, L::TowerRuinsKeep],
        }
    }
}
