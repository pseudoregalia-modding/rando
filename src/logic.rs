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

#[derive(Clone, Copy, PartialEq)]
pub enum Drop {
    Ability(Ability),
    SmallKey,
    BigKey,
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
            Drop::BigKey => f.write_str("Big Key"),
            Drop::Health => f.write_str("Health"),
            Drop::Goatling(_) => f.write_str("Goatling"),
            Drop::Note => f.write_str("Note"),
            Drop::Chair => f.write_str("Chair"),
        }
    }
}

#[derive(Clone)]
pub struct Check {
    description: &'static str,
    pub location: Location,
    pub index: usize,
    pub drop: Drop,
    locks: Lock,
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

#[derive(Debug, Clone, Copy)]
pub enum Lock {
    None,
    Any(&'static [ Lock]),
    All(&'static [Lock]),
    Location(Location),
    Movement(Ability),
    SmallKey,
    Ending,
}

impl Into<Lock> for Ability {
    fn into(self) -> Lock {
        Lock::Movement(self)
    }
}

impl Into<Lock> for Location {
    fn into(self) -> Lock {
        Lock::Location(self)
    }
}
