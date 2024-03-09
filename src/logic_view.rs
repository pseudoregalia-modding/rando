use std::default;

use crate::logic::{
 Ability, Difficulty, Location, Lock, Trick
};
use eframe::egui;
pub struct LogicRead {}
#[derive(
  strum::EnumString,
  strum::Display,
  strum::EnumIter,
  strum::EnumVariantNames,
  strum::AsRefStr,
  Default,
  Copy,
  Clone,
  Debug,
  PartialEq,
  PartialOrd,
)]
pub enum Area {
  #[default]
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
  TowerRuins
}
use Location::*;
impl Area {
  pub fn get_nodes(&self) -> &[Location] {
    match self {
      Area::Dungeon => &[
        VDreamBreaker,
        EarlyPrison,
        LatePrison,
        StrongEyes,
        PEntryCastle,
        PEntryUnderBelly,
        PEntryTheatre,
      ],
      Area::CastleSansa => &[
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
      ],
      Area::SansaKeep => &[
        SkCastleRampEntry,
        SkCastleMainEntry,
        SkCastleClimbEntry,
        SkUnderbellyEntry,
        SkTheatreEntry,
        SansaKeep,
        Sunsetter,
      ],
      Area::ListlessLibrary => &[
        MainLibrary,
        Restricted,
        LibSaveNearGreaves,
      ],
      Area::TwilightTheatre => &[
        ThCastleEntryPillar,
        ThCastleEntryMain,
        ThBaileyEntry,
        ThKeepEntry,
        ThDungeonEntry,
        PillarRoom,
        TheatreEntrance,
        OtherTheatrePath,
        MainTheatre,
      ],
      Area::EmptyBailey => &[
        EmptyBailey,
        EbEntryUnderBelly,
        EbEntryRuins,
        EbEntryTheatre,
        EbEntryCastle,
      ],
      Area::Underbelly => &[
        SansaHole,
        BaileyHole,
        PrisonHole,
        MainUnderbelly,
        VAscendantLight,
        HpSave,
      ],
      Area::TowerRuins => &[
        TowerRuinsEntrance,
        TowerRuinsKeep,
      ]
    }
  }
}


impl LogicRead {
 pub fn get_trick(trick: Trick, diff: Difficulty) -> String {
   match trick {
     Trick::Momentum => return format!("Momentum Conservation at {:?}", diff),
     Trick::Movement => return format!("Movement at {:?}", diff),
     Trick::ClingAbuse => return format!("Cling Abuse at {:?}", diff),
     Trick::OneWall => return format!("Single Wall wall-kick at {:?}", diff),
     Trick::PogoAbuse => return format!("Ascendant Light Abuse at {:?}", diff),
     Trick::ReverseKick => return format!("Reverse Wall Kick at {:?}", diff),
     Trick::SunsetterAbuse => return format!("Sunsetter Backflip abuse at {:?}", diff),
  };
 }

 pub fn get_ability(ability: Ability) -> String {
  match ability {
   Ability::AscendantLight => "Ascendant Light".to_string(),
   Ability::ClingGem(num) => return format!("Cling Gem or {:?} clings", num),
   Ability::DreamBreaker => "Dream Breaker".to_string(),
   Ability::HeliacalPower => "Heliacal Power".to_string(),
   Ability::Indignation => "Indignation".to_string(),
   Ability::Slide => "Slide".to_string(),
   Ability::SolarWind => "Solar Wind".to_string(),
   Ability::SoulCutter => "Soul Cutter".to_string(),
   Ability::Strikebreak => "Strike Break".to_string(),
   Ability::SunGreaves => "Sun Greaves".to_string(),
   Ability::Sunsetter => "Sun Setter".to_string(),
   _ => "".to_string(),
  }
 }

 pub fn logic_node_lock_to_ui(ui: &mut egui::Ui, lock: Lock, id: u8) {
  let ID = id | 0;
   match lock {
    Lock::Any(locks) => {
     let mut new_id = ID + 1;
     egui::CollapsingHeader::new("Any of the following:").id_source(ID)
     .show(ui, |ui| {
      for lock in locks {
       new_id += 1;
       LogicRead::logic_node_lock_to_ui(ui, *lock, new_id);
      }
     });
    },
    Lock::All(locks) => {
     let mut new_id = ID + 1;
     egui::CollapsingHeader::new("All of the following:").id_source(ID)
     .show(ui, |ui| {
      for lock in locks {
       new_id += 1;
       LogicRead::logic_node_lock_to_ui(ui, *lock, new_id);
      }
     });
    },
    Lock::Location(loc) => {ui.label(format!("Access to: {:?}", loc));},
    Lock::Movement(ability) => {ui.label(LogicRead::get_ability(ability));},
    Lock::SmallKey => {ui.label("Small Key");},
    Lock::Trick(trick,diff) =>{ui.label(LogicRead::get_trick(trick, diff));},
    _ => {}
  }
 }
}