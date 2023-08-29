mod drop;
pub use drop::*;
mod checks;
pub use checks::CHECKS;
mod seeding;
pub use seeding::randomise;
mod locations;
pub use locations::Locations;

#[derive(Debug)]
pub struct Data {
    pub overworld: std::collections::HashMap<Locations, Vec<Check>>,
}

#[derive(Clone, Copy, Debug, strum::AsRefStr, PartialEq)]
pub enum Drop {
    Ability(Ability),
    SmallKey,
    BigKey,
    Health,
}

#[derive(Debug)]
pub struct Check {
    pub location: Locations,
    pub name: &'static str,
    pub drop: Drop,
    locks: &'static [Lock],
}

#[derive(Debug)]
pub enum Lock {
    Location(Locations),
    Movement(&'static [Ability]),
    SmallKey,
    Ending,
}
