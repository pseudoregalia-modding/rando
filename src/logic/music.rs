#[derive(Clone)]
pub enum Music {
    DilapidatedDungeon,
    StrongEyes,
    CastleSansa,
    SansaKeep,
    EmptyBailey,
    TowerRuins,
    Underbelly,
    TwilightTheatre,
    Princess,
}

use Music as M;

impl Music {
    pub const fn track(&self) -> &'static str {
        match self {
            M::DilapidatedDungeon => "BGM_Dungeon",
            M::StrongEyes => "BGM_Boss1",
            M::CastleSansa => "BGM_LowerCastle",
            M::SansaKeep => "BGM_UpperCastle",
            M::EmptyBailey => "BGM_Exterior",
            M::TowerRuins => "BGM_Tower",
            M::Underbelly => "BGM_Caves",
            M::TwilightTheatre => "BGM_Theatre",
            M::Princess => "BGM_BossPrincess",
        }
    }
    pub const fn source(&self) -> &'static str {
        match self {
            M::DilapidatedDungeon => "gg_exterior",
            M::StrongEyes => "final_boss_theme_in_game",
            M::CastleSansa => "gg_lower_castle",
            M::SansaKeep => "gg_lower_castle_alt2",
            M::EmptyBailey => "gg_upper_castle",
            M::TowerRuins => "matjam_1_final",
            M::Underbelly => "matjam_2_final",
            M::TwilightTheatre => "gg_theater",
            M::Princess => "RITTZGAM2",
        }
    }
    pub const fn volume(&self) -> f32 {
        match self {
            M::DilapidatedDungeon => 1.0,
            M::StrongEyes => 1.4,
            M::CastleSansa => 0.3,
            M::SansaKeep => 0.2,
            M::EmptyBailey => 1.0,
            M::TowerRuins => 1.0,
            M::Underbelly => 1.0,
            M::TwilightTheatre => 0.4,
            M::Princess => 1.2,
        }
    }
}
