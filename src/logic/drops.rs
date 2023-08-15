#![allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Items {
    #[strum(serialize = "0")]
    LargePouch,
    #[strum(serialize = "1")]
    SmallPouch,
    #[strum(serialize = "6")]
    OldKey,
    #[strum(serialize = "12")]
    Null,
    #[strum(serialize = "15")]
    KinbankDebitCard,
    #[strum(serialize = "18")]
    SanctuaryStone,
    #[strum(serialize = "19")]
    RareKey,
    #[strum(serialize = "32")]
    ShadowFragment,
    #[strum(serialize = "22")]
    PureShadowCatcher,
    #[strum(serialize = "33")]
    BlackFire,
    #[strum(serialize = "35")]
    Coin,
    #[strum(serialize = "36")]
    VoidShards,
    #[strum(serialize = "41")]
    HolyBlessing,
    #[strum(serialize = "49")]
    SpiritCatcher,
    #[strum(serialize = "50")]
    HouseKey,
    #[strum(serialize = "23")]
    Null2,
    #[strum(serialize = "39")]
    FireEssence,
    #[strum(serialize = "7")]
    Book,
    #[strum(serialize = "14")]
    Boot,
    #[strum(serialize = "17")]
    IceCrystal,
    #[strum(serialize = "24")]
    RubyOre,
    #[strum(serialize = "25")]
    SapphireOre,
    #[strum(serialize = "31")]
    EmeraldOre,
    #[strum(serialize = "27")]
    BremurPicture,
    #[strum(serialize = "28")]
    OddRock,
    #[strum(serialize = "29")]
    Souls,
    #[strum(serialize = "30")]
    SandRelic,
    #[strum(serialize = "9")]
    Rose,
    #[strum(serialize = "34")]
    AbyssPotion,
    #[strum(serialize = "40")]
    ShadowPotion,
    #[strum(serialize = "44")]
    CarrotPotion,
    #[strum(serialize = "42")]
    Rice,
    #[strum(serialize = "45")]
    Apple,
    #[strum(serialize = "46")]
    RottenApple,
    #[strum(serialize = "47")]
    Medicine,
    #[strum(serialize = "51")]
    LifeElixir,
    #[strum(serialize = "52")]
    RoyalElixir,
    #[strum(serialize = "53")]
    BoulderPowder,
    #[strum(serialize = "54")]
    RareCheese,
    #[strum(serialize = "55")]
    SeagulSoup,
    #[strum(serialize = "56")]
    FleshEater,
    #[strum(serialize = "57")]
    ShardCluster,
    #[strum(serialize = "58")]
    ForestBug,
    #[strum(serialize = "26")]
    DeadRat,
    #[strum(serialize = "59")]
    PoisonedPlant,
    #[strum(serialize = "60")]
    Null3,
    #[strum(serialize = "61")]
    Dash,
    #[strum(serialize = "62")]
    DoubleJump,
    #[strum(serialize = "63")]
    SpinAttack,
    #[strum(serialize = "64")]
    WallRun,
    #[strum(serialize = "65")]
    FireBall,
    #[strum(serialize = "66")]
    DownSmash,
    #[strum(serialize = "67")]
    Shield,
    #[strum(serialize = "68")]
    SpiritSlot,
    #[strum(serialize = "69")]
    VoidKey,
    #[strum(serialize = "70")]
    Necklace,
    #[strum(serialize = "71")]
    KeyFireMaster,
    #[strum(serialize = "72")]
    KeyHolyMaster,
    #[strum(serialize = "73")]
    KeyIceMaster,
    #[strum(serialize = "74")]
    KeyDeathMaster,
    #[strum(serialize = "75")]
    KeyUthasTemple,
    #[strum(serialize = "76")]
    KeyGodMaster,
    #[strum(serialize = "77")]
    KeySteam,
    #[strum(serialize = "78")]
    KeyGraveyardKey,
    #[strum(serialize = "79")]
    HouseContract,
    #[strum(serialize = "80")]
    Mandoline,
    #[strum(serialize = "81")]
    RareGlasses,
    #[strum(serialize = "83")]
    RareSnow,
    #[strum(serialize = "84")]
    ComposerLetter,
    #[strum(serialize = "85")]
    Sprint,
    #[strum(serialize = "86")]
    BeiraVessel,
    #[strum(serialize = "87")]
    BeiraShards,
    #[strum(serialize = "88")]
    BasicPouch,
    #[strum(serialize = "89")]
    FireEssenceSlot,
    #[strum(serialize = "90")]
    VoidOre,
    #[strum(serialize = "91")]
    ExtraLargePouch,
    #[strum(serialize = "92")]
    Mana,
    #[strum(serialize = "93")]
    GuardianSoul,
    #[strum(serialize = "94")]
    CovenantSoul,
    #[strum(serialize = "95")]
    GuardianKey,
    #[strum(serialize = "96")]
    Duck,
    #[strum(serialize = "97")]
    RobiBadge,
}

impl Items {
    pub fn treasure(&self) -> bool {
        matches!(
            self,
            Items::Apple
                | Items::Boot
                | Items::FleshEater
                | Items::IceCrystal
                | Items::Mandoline
                | Items::RareCheese
                | Items::RareGlasses
                | Items::Rice
                | Items::SandRelic
                | Items::SeagulSoup
        )
    }

    pub fn gem(&self) -> bool {
        matches!(
            self,
            Items::RubyOre | Items::SapphireOre | Items::EmeraldOre | Items::VoidOre
        )
    }

    pub fn key(&self) -> bool {
        matches!(
            self,
            Items::OldKey
                | Items::KeyGraveyardKey
                | Items::KeyUthasTemple
                | Items::KeyHolyMaster
                | Items::KeyGodMaster
                | Items::KeySteam
                | Items::KeyFireMaster
        )
    }

    pub fn key_item(&self) -> bool {
        matches!(
            self,
            Items::BasicPouch
                | Items::SmallPouch
                | Items::LargePouch
                | Items::ExtraLargePouch
                | Items::OldKey
                | Items::SanctuaryStone
                | Items::HouseKey
                | Items::BremurPicture
                | Items::Rose
                | Items::Necklace
                | Items::Book
                | Items::ComposerLetter
                | Items::OddRock
                | Items::BeiraVessel
                | Items::RareSnow
                | Items::FireEssenceSlot
                | Items::HouseContract
                | Items::KeyGraveyardKey
                | Items::KeyUthasTemple
                | Items::KeyHolyMaster
                | Items::KeyGodMaster
                | Items::KeySteam
                | Items::KeyFireMaster
                | Items::Shield
                | Items::FireBall
                | Items::WallRun
                | Items::DoubleJump
                | Items::SpinAttack
                | Items::Sprint,
        )
    }
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Weapons {
    #[strum(serialize = "0")]
    DualBlades,
    #[strum(serialize = "1")]
    BloodstormBlades,
    #[strum(serialize = "2")]
    DiamondWings,
    #[strum(serialize = "3")]
    ShadowCasters,
    #[strum(serialize = "4")]
    EmberTwins,
    #[strum(serialize = "5")]
    IronJustice,
    #[strum(serialize = "6")]
    IceDestroyers,
    #[strum(serialize = "7")]
    PeaceKeepers,
    #[strum(serialize = "8")]
    SteelShanks,
    #[strum(serialize = "9")]
    BremurFamilySwords,
    #[strum(serialize = "10")]
    SilverBlades,
    #[strum(serialize = "11")]
    KinaDefenders,
    #[strum(serialize = "12")]
    Empty,
    #[strum(serialize = "13")]
    DLCVoidMaster,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Tunics {
    #[strum(serialize = "0")]
    ShadowCloack,
    #[strum(serialize = "1")]
    FireGarment,
    #[strum(serialize = "2")]
    OnopCoat,
    #[strum(serialize = "3")]
    PerformerCostume,
    #[strum(serialize = "4")]
    MerchantsRobe,
    #[strum(serialize = "5")]
    BunnySuit,
    #[strum(serialize = "6")]
    ForestTunic,
    #[strum(serialize = "7")]
    PureShadow,
    #[strum(serialize = "8")]
    SilverCloack,
    #[strum(serialize = "9")]
    GoldenRobe,
    #[strum(serialize = "10")]
    SteamWorkerTunic,
    #[strum(serialize = "11")]
    ThiefsCloack,
    #[strum(serialize = "12")]
    Empty,
    #[strum(serialize = "13")]
    SectMember,
    #[strum(serialize = "14")]
    Pumpkin,
    #[strum(serialize = "15")]
    Galaxy,
    #[strum(serialize = "16")]
    BananaKing,
    #[strum(serialize = "17")]
    Red,
    #[strum(serialize = "18")]
    Yellow,
    #[strum(serialize = "19")]
    Green,
    #[strum(serialize = "20")]
    Grey,
    #[strum(serialize = "21")]
    Violet,
    #[strum(serialize = "22")]
    LightBlue,
    #[strum(serialize = "23")]
    Rainbow,
    #[strum(serialize = "24")]
    Lila,
    #[strum(serialize = "25")]
    Royal,
    #[strum(serialize = "26")]
    Aqua,
    #[strum(serialize = "27")]
    Orange,
    #[strum(serialize = "28")]
    DLCVoidMaster,
    #[strum(serialize = "29")]
    DLCDuck,
    #[strum(serialize = "30")]
    DLCCursedDuck,
    #[strum(serialize = "31")]
    AlphaUmbra,
    #[strum(serialize = "32")]
    DiscordWinnerContest,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Spirits {
    #[strum(serialize = "0")]
    FarasGrace,
    #[strum(serialize = "1")]
    HammerKing,
    #[strum(serialize = "2")]
    HolyCentry,
    #[strum(serialize = "3")]
    RiverSpirit,
    #[strum(serialize = "4")]
    AngryAmbusher,
    #[strum(serialize = "5")]
    SecretFruit,
    #[strum(serialize = "6")]
    MindController,
    #[strum(serialize = "7")]
    FrozenSoul,
    #[strum(serialize = "8")]
    HowlingTree,
    #[strum(serialize = "9")]
    LoveFlower,
    #[strum(serialize = "10")]
    StormCentry,
    #[strum(serialize = "11")]
    BloodPhantom,
    #[strum(serialize = "12")]
    PossesedBook,
    #[strum(serialize = "13")]
    ForestGuardian,
    #[strum(serialize = "15")]
    MoiTheDreadful,
    #[strum(serialize = "16")]
    StoneHunter,
    #[strum(serialize = "17")]
    GoldenLust,
    #[strum(serialize = "18")]
    SpringWarrior,
    #[strum(serialize = "14")]
    OnopSiblings,
    #[strum(serialize = "19")]
    CandleOnop,
    #[strum(serialize = "20")]
    StoneWarrior,
    #[strum(serialize = "21")]
    ToxicRat,
    #[strum(serialize = "22")]
    SummonedGod,
    #[strum(serialize = "23")]
    SummoningHand,
    #[strum(serialize = "24")]
    BettingHand,
    #[strum(serialize = "25")]
    LifeSteal,
    #[strum(serialize = "26")]
    ShadowDemon,
    #[strum(serialize = "27")]
    ShadowGru,
    #[strum(serialize = "28")]
    FlyingOnop,
    #[strum(serialize = "29")]
    ToxicWater,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Abilities {
    #[strum(serialize = "3")]
    Attack,
    #[strum(serialize = "2")]
    Dash,
    #[strum(serialize = "1")]
    DoubleJump,
    #[strum(serialize = "9")]
    WallRun,
    #[strum(serialize = "11")]
    Sprint,
    #[strum(serialize = "4")]
    DownSmash,
    #[strum(serialize = "13")]
    Spell,
    #[strum(serialize = "10")]
    Grind,
    #[strum(serialize = "14")]
    Block,
    #[strum(serialize = "15")]
    SpinAttack,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Emotes {
    #[strum(serialize = "0")]
    Wave,
    #[strum(serialize = "1")]
    Applause,
    #[strum(serialize = "2")]
    Levitation,
    #[strum(serialize = "3")]
    Windmill,
    #[strum(serialize = "4")]
    HatKid,
    #[strum(serialize = "5")]
    Triceps,
    #[strum(serialize = "6")]
    Aggressive,
    #[strum(serialize = "7")]
    No,
    #[strum(serialize = "8")]
    Photo,
    #[strum(serialize = "9")]
    Celebration,
    #[strum(serialize = "11")]
    KungFu,
    #[strum(serialize = "12")]
    Techno,
    #[strum(serialize = "13")]
    Party,
    #[strum(serialize = "14")]
    Hello2,
    #[strum(serialize = "15")]
    Empty,
}
