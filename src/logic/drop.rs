#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Ability {
    // #[strum(serialize = "attack")]
    // DreamBreaker,
    #[strum(serialize = "airKick")]
    SunGreaves,
    #[strum(serialize = "slide")]
    Slide,
    #[strum(serialize = "plunge")]
    Sunsetter,
    #[strum(serialize = "wallRide")]
    ClingGem,
    #[strum(serialize = "light")]
    AscendantLight,
    #[strum(serialize = "projectile")]
    SoulCutter,
    #[strum(serialize = "powerBoost")]
    Indignation,
    #[strum(serialize = "SlideJump")]
    SolarWind,
    #[strum(serialize = "chargeAttack")]
    Strikebreak,
    #[strum(serialize = "extraKick")]
    HeliacalPower,
    #[strum(serialize = "airRecovery")]
    AerialFinesse,
    #[strum(serialize = "mobileHeal")]
    Pilgrimage,
    #[strum(serialize = "magicHaste")]
    Empathy,
    #[strum(serialize = "HealBoost")]
    GoodGraces,
    #[strum(serialize = "damageBoost")]
    MartialProwess,
    #[strum(serialize = "magicPiece")]
    ClearMind,
    #[strum(serialize = "outfitPro")]
    Professional,
}
