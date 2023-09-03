mod drop;
pub use drop::*;
mod checks;
pub use checks::CHECKS;
mod seeding;
pub use seeding::randomise;
mod locations;
pub use locations::Location;

#[derive(Debug)]
pub struct Data {
    pub overworld: std::collections::BTreeMap<&'static str, Vec<Check>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    locks: &'static [Lock],
}

#[derive(Debug)]
pub enum Lock {
    Location(Location),
    Movement(&'static [&'static [Ability]]),
    SmallKey,
    Ending,
}
