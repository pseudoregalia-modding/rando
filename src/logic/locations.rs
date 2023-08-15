use super::*;
use crate::{no_walljump, walljump};

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display, strum::EnumIter, strum::EnumCount)]
pub enum Locations {
    // Fire Keep
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth")]
    Lab,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_Exterior")]
    Bitoven,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_DLC_VoidMaster")]
    KeepDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepEast")]
    KeepVessel,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_EastWing")]
    Shield,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom")]
    FirstVoid,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepWest")]
    Crates,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_MemorialMain")]
    Memorial,
    // Arcane Tunnels
    #[strum(serialize = "A02_ArcaneTunnels/A02_NorthArcane")]
    ArcaneNorth,
    #[strum(serialize = "A02_ArcaneTunnels/A02_SouthArcane")]
    ArcaneSouth,
    #[strum(serialize = "A02_ArcaneTunnels/A02_EastArcane")]
    ArcaneSpiritHunter,
    #[strum(serialize = "A02_ArcaneTunnels/A02_Arcane")]
    ArcaneDucks,
    // Crossroads
    #[strum(serialize = "A01_StoneHeartCity/A01_CrossRoads")]
    Crossroads,
    #[strum(serialize = "A01_StoneHeartCity/A01_Well")]
    Well,
    // Stoneheart City
    #[strum(serialize = "A01_StoneHeartCity/A01_CliffPath")]
    Stoneheart,
    // Forest Temple
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels")]
    WaterLevels,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone")]
    NuosClaw,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_CenterTree")]
    Tree,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine")]
    ForestDucks,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_BossRoom")]
    Gruh,
    // Temple Gardens
    #[strum(serialize = "A01_StoneHeartCity/A01_TempleGardens")]
    TempleGardens,
    // Abandoned Path
    #[strum(serialize = "A01_StoneHeartCity/A01_Graveyard")]
    AbandonedPath,
    #[strum(serialize = "A01_StoneHeartCity/A01_GraveyardShrine")]
    Beira,
    // Uthas Temple
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_Intro")]
    UthasStart,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_Main")]
    UthasBracelet,
    #[strum(serialize = "A01_StoneHeartCity/A01_SmallShrine")]
    UthasDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_SouthEast")]
    UthasPuzzle,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_SouthWest")]
    UthasCombat,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_BottomPassage")]
    UthasPlatforming,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_EndPath")]
    UthasEnd,
    // Temple of the gods
    #[strum(serialize = "A10_PenumbraTemple/A10_Entrance")]
    SanctuaryStone,
    #[strum(serialize = "A10_PenumbraTemple/A10_GodessChamber")]
    Queen,
    // Firefall River
    #[strum(serialize = "A06_IronCaves/A06_FireFall_A")]
    FirefallSpiritHunter,
    #[strum(serialize = "A06_IronCaves/A06_FireFall_B")]
    Bunny,
    #[strum(serialize = "A06_IronCaves/A06_LakeMolva")]
    LakeMolva,
    #[strum(serialize = "A06_IronCaves/A06_FireFallRiver")]
    FirefallDucks,
    // Steam House
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse_Core")]
    SteamHouse,
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse")]
    SteamHouseDucks,
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse_Corridor")]
    SteamHousePlatforming,
    #[strum(serialize = "A06_IronCaves/A06_Sirion")]
    Sirion,
    // Rust Village
    #[strum(serialize = "A06_IronCaves/A06_RustCity")]
    RustVillage,
    // Waterway
    #[strum(serialize = "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess")]
    Waterway,
    #[strum(serialize = "A02_ArcaneTunnels/A02_CentralWaterWay")]
    WaterwayDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A02_BossRoom")]
    Samael,
    #[strum(serialize = "Voids/16_VoidPaul/Void_Paul")]
    PathOfVictory,
    #[strum(serialize = "Voids/03_VoidEasy/Void_Easy")]
    ArigosChallenge,
    #[strum(serialize = "Voids/02_Void04_V2/Void_04_V2")]
    BorisasFate,
    #[strum(serialize = "Voids/15_VoidFlauta/Void_Flauta")]
    JuliansSong,
    #[strum(serialize = "Voids/17_VoidSanti/Void_Santi")]
    Alchemist,
    #[strum(serialize = "don't look into it - after all it stares back")]
    TheVoid,
}

impl Locations {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            Locations::Lab => &[&[]],
            Locations::Bitoven => &[&[Lock::Location(Locations::Lab)]],
            Locations::KeepDucks => &[&[Lock::Location(Locations::Bitoven)]],
            Locations::KeepVessel => &[&[Lock::Location(Locations::Lab)]],
            Locations::Shield => &[&[Lock::Location(Locations::KeepVessel)]],
            Locations::FirstVoid => &[&[Lock::Location(Locations::Lab), Lock::Item(Items::OldKey)]],
            Locations::Crates => &[&[Lock::Location(Locations::FirstVoid)]],
            Locations::Memorial => &[&[Lock::Location(Locations::Crates)]],
            Locations::ArcaneNorth => &[&[Lock::Location(Locations::Memorial)]],
            Locations::ArcaneSouth => &[&[Lock::Location(Locations::ArcaneNorth)]],
            Locations::ArcaneSpiritHunter => &[&[Lock::Location(Locations::ArcaneSouth)]],
            Locations::ArcaneDucks => &[&[Lock::Location(Locations::ArcaneSpiritHunter)]],
            Locations::Crossroads => &[&[Lock::Location(Locations::ArcaneSpiritHunter)]],
            Locations::Well => &[&[
                Lock::Location(Locations::Crossroads),
                Lock::Movement(&[no_walljump!(0, 1)]),
            ]],
            Locations::Stoneheart => &[&[Lock::Location(Locations::ArcaneSpiritHunter)]],
            Locations::WaterLevels => &[&[Lock::Location(Locations::Stoneheart)]],
            Locations::NuosClaw => &[&[
                Lock::Location(Locations::WaterLevels),
                Lock::Item(Items::OldKey),
            ]],
            Locations::Tree => &[&[
                Lock::Location(Locations::WaterLevels),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(2, 0)]),
            ]],
            Locations::ForestDucks => &[&[Lock::Location(Locations::Tree)]],
            Locations::Gruh => &[&[
                Lock::Location(Locations::Tree),
                Lock::Item(Items::KeyHolyMaster),
            ]],
            Locations::TempleGardens => &[&[
                Lock::Item(Items::OldKey),
                Lock::Item(Items::KeyHolyMaster),
                Lock::Location(Locations::UthasEnd),
                Lock::Movement(&[no_walljump!(1, 0)]),
            ]],
            Locations::AbandonedPath => &[&[
                Lock::Location(Locations::Stoneheart),
                Lock::Item(Items::KeyGraveyardKey),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
            ]],
            Locations::Beira => &[&[
                Lock::Location(Locations::AbandonedPath),
                Lock::Location(Locations::LakeMolva),
                Lock::Location(Locations::TempleGardens),
                Lock::Item(Items::SanctuaryStone),
                Lock::Item(Items::BeiraVessel),
                // needs walljump for temple gardens blocked stairway soul
                // also requires climbing tower
                Lock::Movement(&[walljump!(3, 3)]),
            ]],
            Locations::UthasStart => &[&[
                Lock::Location(Locations::AbandonedPath),
                Lock::Movement(&[no_walljump!(0, 2)]),
                Lock::Item(Items::KeyUthasTemple),
            ]],
            Locations::UthasBracelet => &[&[
                Lock::Location(Locations::UthasStart),
                Lock::Item(Items::OldKey),
            ]],
            Locations::UthasDucks => &[&[Lock::Location(Self::UthasBracelet)]],
            Locations::UthasPuzzle => &[&[Lock::Location(Self::UthasBracelet)]],
            Locations::UthasCombat => &[&[
                Lock::Location(Locations::UthasBracelet),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
                Lock::Item(Items::OldKey),
            ]],
            Locations::UthasPlatforming => &[&[
                Lock::Location(Locations::UthasBracelet),
                Lock::Movement(&[no_walljump!(1, 4), walljump!(1, 3)]),
                Lock::Item(Items::OldKey),
            ]],
            Locations::UthasEnd => &[
                &[
                    Lock::Location(Locations::UthasBracelet),
                    Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
                    Lock::Item(Items::OldKey),
                ],
                &[
                    Lock::Location(Locations::UthasBracelet),
                    Lock::Movement(&[walljump!(0, 1), no_walljump!(4, 3)]),
                ],
            ],
            Locations::SanctuaryStone => &[&[
                Lock::Location(Locations::TempleGardens),
                Lock::Item(Items::KeyGodMaster),
            ]],
            Locations::Queen => &[&[
                Lock::Location(Locations::SanctuaryStone),
                Lock::Location(Locations::Samael),
                Lock::Location(Locations::Sirion),
                Lock::Location(Locations::Beira),
            ]],
            Locations::FirefallSpiritHunter => &[&[Lock::Location(Locations::SanctuaryStone)]],
            Locations::Bunny => &[&[
                Lock::Location(Self::FirefallSpiritHunter),
                Lock::Movement(&[no_walljump!(0, 3)]),
            ]],
            Locations::LakeMolva => &[&[
                Lock::Location(Locations::Bunny),
                Lock::Movement(&[no_walljump!(0, 1)]),
            ]],
            Locations::FirefallDucks => &[&[Lock::Location(Locations::LakeMolva)]],
            Locations::SteamHouse => &[&[
                Lock::Location(Locations::LakeMolva),
                Lock::Movement(&[no_walljump!(1, 2)]),
            ]],
            Locations::SteamHouseDucks => &[&[Lock::Location(Locations::SteamHouse)]],
            Locations::SteamHousePlatforming => &[&[
                Lock::Location(Locations::SteamHouse),
                Lock::Item(Items::KeySteam),
                Lock::Movement(&[no_walljump!(1, 4), walljump!(1, 3)]),
            ]],
            Locations::Sirion => &[&[
                Lock::Location(Locations::SteamHouse),
                Lock::Item(Items::KeyFireMaster),
                Lock::Movement(&[walljump!(0, 4), no_walljump!(0, 5)]),
                Lock::Item(Items::SanctuaryStone),
            ]],
            Locations::RustVillage => &[&[
                Lock::Location(Locations::SteamHouse),
                Lock::Location(Locations::SteamHousePlatforming),
                Lock::Movement(&[no_walljump!(1, 2)]),
                Lock::IronJustice,
            ]],
            Locations::Waterway => &[&[Lock::Location(Locations::ArcaneSouth)]],
            Locations::WaterwayDucks => &[&[Lock::Location(Locations::Waterway)]],
            Locations::Samael => &[&[
                Lock::Location(Locations::ArcaneSouth),
                Lock::Location(Locations::LakeMolva),
                // movement requirements for oliver's diary area
                Lock::Movement(&[walljump!(0, 4), no_walljump!(0, 8)]),
                Lock::Item(Items::SanctuaryStone),
            ]],
            Locations::PathOfVictory => &[&[Lock::Location(Locations::ArcaneSouth)]],
            Locations::ArigosChallenge => &[&[Lock::Location(Locations::Tree)]],
            Locations::BorisasFate => &[&[
                Lock::Location(Locations::Stoneheart),
                Lock::Movement(&[walljump!(0, 2), no_walljump!(0, 4)]),
            ]],
            Locations::JuliansSong => &[&[Lock::Location(Locations::TempleGardens)]],
            Locations::Alchemist => &[&[Lock::Location(Locations::LakeMolva)]],
            Locations::TheVoid => &[&[
                Lock::Location(Locations::PathOfVictory),
                Lock::Location(Locations::ArigosChallenge),
                Lock::Location(Locations::BorisasFate),
                Lock::Location(Locations::JuliansSong),
                Lock::Location(Locations::Alchemist),
                Lock::Movement(&[walljump!(1, 3)]),
            ]],
        }
    }
}
