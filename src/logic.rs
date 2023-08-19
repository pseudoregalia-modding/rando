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

#[derive(Debug)]
pub enum Context {
    Overworld(&'static str),
}

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
pub enum Abilities {}

#[derive(Clone, Copy, Debug, strum::AsRefStr, PartialEq)]
pub enum Drop {
    Ability(Abilities),
    SmallKey,
    BigKey,
    Health,
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
    Movement(&'static [Abilities]),
}
