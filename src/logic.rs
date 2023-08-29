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
    pub overworld: std::collections::BTreeMap<Location, Vec<Check>>,
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
    pub name: &'static str,
    pub drop: Drop,
    locks: &'static [Lock],
}

#[derive(Debug)]
pub enum Lock {
    Location(&'static [Location]),
    Movement(&'static [Ability]),
    SmallKey,
    Ending,
}
