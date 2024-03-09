#[derive(Copy, Clone, Debug)]
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

pub struct Tricks {
    pub momentum: Difficulty,        // Momentum conservation
    pub one_wall: Difficulty,        // Single Wall wall-kicks
    pub reverse_kick: Difficulty,    // Reverse wall kicks
    pub sunsetter_abuse: Difficulty, // Abuse the backflip
    pub pogo_abuse: Difficulty,      // Ascendant Light
    pub movement: Difficulty,        // General movement such as backflips
    pub cling_abuse: Difficulty,     // Abuse of cling to climb corners
}