mod drops;
pub use drops::*;
mod checks;
pub use checks::CHECKS;
mod seeding;
pub use seeding::randomise;
mod locations;
pub use locations::*;

#[derive(Debug)]
pub struct Data {
    pub overworld: std::collections::HashMap<Locations, Vec<Check>>,
    pub cutscenes: Vec<Check>,
    pub savegames: Vec<Check>,
    pub cases: Vec<Check>,
    pub shop_emotes: Vec<(Shop, usize)>,
}

#[derive(Debug, Clone, Copy, strum::EnumIter, strum::AsRefStr)]
pub enum Shop {
    #[strum(serialize = "ShopA")]
    Mork = 7,
    #[strum(serialize = "ShopC")]
    SpiritHunter = 9,
    #[strum(serialize = "ShopD")]
    Ari = 10,
    #[strum(serialize = "ShopE")]
    Poti = 11,
    #[strum(serialize = "ShopF")]
    Poi = 12,
    #[strum(serialize = "ShopH")]
    Nilo = 19,
}

impl Shop {
    pub fn location(&self) -> unreal_asset::types::vector::Vector<f64> {
        use unreal_asset::types::vector::Vector;
        match self {
            Shop::Mork => Vector::new(52459.4, -17121.4, -35.13),
            Shop::SpiritHunter => Vector::new(38096.7, -4988.86, -38178.1),
            Shop::Ari => Vector::new(28906.2, -6509.37, 5948.82),
            Shop::Poti => Vector::new(-94159.0, -6261.17, -28737.0),
            Shop::Poi => Vector::new(21144.0, -39103.5, 5637.0),
            Shop::Nilo => Vector::new(-176.74, 36323.6, 8701.8),
        }
    }
}

#[derive(Debug, strum::AsRefStr)]
pub enum Case {
    #[strum(serialize = "NPC/Bremur/NPC_Bremur")]
    Bremur,
    #[strum(serialize = "NPC/Onops/MUSIC_Onops/Onop_Compositor/NPC_Onop_Compositor_Ready")]
    Paulale,
    #[strum(serialize = "InteractiveObjects/Collectibles/BloodStone/BloodStone_BP")]
    Angels,
    #[strum(serialize = "Player/Logic/Player_Character_BP")]
    AllVoids,
}

#[derive(Debug)]
pub enum Context {
    Shop(Shop, usize, i32),
    Cutscene(&'static str),
    Overworld(&'static str),
    Starting,
    Specific(Case, usize),
}

#[derive(Clone, Copy, Debug, strum::AsRefStr)]
pub enum Drop {
    #[strum(serialize = "0")]
    Item(Items, i32),
    #[strum(serialize = "1")]
    Weapon(Weapons),
    #[strum(serialize = "2")]
    Tunic(Tunics),
    #[strum(serialize = "3")]
    Spirit(Spirits),
    #[strum(serialize = "6")]
    Ability(Abilities),
    #[strum(serialize = "7")]
    Emote(Emotes),
    #[strum(serialize = "0")]
    Ore(i32),
    #[strum(serialize = "0")]
    Duck,
}

impl PartialEq for Drop {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // ignore amount
            (Self::Item(l0, ..), Self::Item(r0, ..)) => l0 == r0,
            (Self::Weapon(l0), Self::Weapon(r0)) => l0 == r0,
            (Self::Tunic(l0), Self::Tunic(r0)) => l0 == r0,
            (Self::Spirit(l0), Self::Spirit(r0)) => l0 == r0,
            (Self::Ability(l0), Self::Ability(r0)) => l0 == r0,
            (Self::Emote(l0), Self::Emote(r0)) => l0 == r0,
            (Self::Ore(l0), Self::Ore(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Drop {
    pub fn as_u8(&self) -> u8 {
        match self {
            Drop::Item(..) => 0,
            Drop::Weapon(..) => 1,
            Drop::Tunic(..) => 2,
            Drop::Spirit(..) => 3,
            Drop::Ability(..) => 6,
            Drop::Emote(..) => 7,
            Drop::Ore(..) => 8,
            Drop::Duck => 9,
        }
    }

    pub fn inner_as_u8(&self) -> u8 {
        match self {
            Drop::Item(inner, _) => *inner as u8,
            Drop::Weapon(inner) => *inner as u8,
            Drop::Tunic(inner) => *inner as u8,
            Drop::Spirit(inner) => *inner as u8,
            Drop::Ability(inner) => *inner as u8,
            Drop::Emote(inner) => *inner as u8,
            Drop::Ore(inner) => *inner as u8,
            Drop::Duck => 80,
        }
    }
}

#[derive(Debug)]
pub struct Check {
    pub location: Locations,
    pub context: Context,
    pub drop: Drop,
    locks: &'static [Lock],
}

#[derive(Debug)]
pub enum Lock {
    Location(Locations),
    Movement(&'static [Move]),
    Item(Items),
    Emote(Emotes),
    Money(u32),
    SpiritHunter,
    Mork,
    EvolairTunic,
    IronJustice,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Move {
    extra_height: u8,
    horizontal: u8,
    walljump: bool,
}

// for some stupid reason const fn doesn't work in slices so i'll use macros instead
#[macro_export]
macro_rules! walljump {
    ($eh: expr, $h: expr) => {
        Move {
            extra_height: $eh,
            horizontal: $h,
            walljump: true,
        }
    };
}
#[macro_export]
macro_rules! no_walljump {
    ($eh: expr, $h: expr) => {
        Move {
            extra_height: $eh,
            horizontal: $h,
            walljump: false,
        }
    };
}
