mod drop;
pub use drop::*;
mod checks;
pub use checks::CHECKS;
mod seeding;
pub use seeding::randomise;
mod locations;
pub use locations::Location;
mod spawns;
pub use spawns::SPAWNS;
mod tricks;
pub use tricks::*;
mod music;
pub use music::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Drop {
    Ability(Ability),
    SmallKey,
    BigKey(i32),
    Health,
    Goatling(&'static [&'static str]),
    Note,
    Chair,
}

impl std::fmt::Debug for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Drop::Ability(a) => f.write_fmt(format_args!("{:?}", a)),
            Drop::SmallKey => f.write_str("Small Key"),
            Drop::BigKey(_) => f.write_str("Big Key"),
            Drop::Health => f.write_str("Health"),
            Drop::Goatling(_) => f.write_str("Goatling"),
            Drop::Note => f.write_str("Note"),
            Drop::Chair => f.write_str("Chair"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Check {
    pub description: &'static str,
    pub location: Location,
    pub index: usize,
    pub drop: Drop,
    pub trial: Option<usize>,
    pub locks: Lock,
}

impl std::fmt::Debug for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}: {} in {}",
            self.drop,
            self.description,
            self.location.name(),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lock {
    None,
    Any(&'static [Lock]),
    All(&'static [Lock]),
    Trick(Trick, Difficulty),
    Location(Location),
    Movement(Ability),
    SmallKey,
    Ending,
}
