use super::*;

pub const CHECKS: [Check; 3] = [
    Check {
        location: Location::DilapidatedDungeon,
        index: 355,
        drop: Drop::Ability(Ability::DreamBreaker),
        locks: &[Lock::Movement(&[Ability::DreamBreaker])],
    },
    Check {
        location: Location::DilapidatedDungeon,
        index: 214,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[Ability::Slide])],
    },
    Check {
        location: Location::DilapidatedDungeon,
        index: 356,
        drop: Drop::Ability(Ability::Slide),
        locks: &[Lock::Movement(&[Ability::DreamBreaker])],
    },
];
