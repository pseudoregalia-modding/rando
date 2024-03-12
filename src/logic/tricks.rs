#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Trick {
    Momentum,       // Momentum conservation
    OneWall,        // Single Wall wall-kicks
    ReverseKick,    // Reverse wall kicks
    SunsetterAbuse, // Abuse the backflip
    PogoAbuse,      // Ascendant Light
    Movement,       // General movement such as backflips
    ClingAbuse,     // Abuse of cling to climb corners
}

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
pub enum Difficulty {
    #[default]
    Disabled,
    Normal,
    Advanced,
    Expert,
    Insane,
}
