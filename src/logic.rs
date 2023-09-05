mod drop;
pub use drop::*;
mod checks;
pub use checks::CHECKS;
mod seeding;
pub use seeding::randomise;
mod locations;
pub use locations::Location;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drop {
    Ability(Ability),
    SmallKey,
    BigKey,
    Health,
}

#[derive(Debug)]
pub struct Check {
    pub location: Location,
    pub index: usize,
    pub drop: Drop,
    locks: &'static [&'static [Lock]],
}

#[derive(Debug, Clone, Copy)]
pub enum Lock {
    Location(Location),
    Movement(&'static [&'static [Ability]]),
    SmallKey,
    Ending,
}
