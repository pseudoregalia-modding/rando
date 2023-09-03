use super::*;

pub const CHECKS: [Check; 23] = [
    // not listing dream breaker since there's nothing it could be replaced by
    // the first thing you see
    Check {
        location: Location::Prison,
        index: 214,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::Sunsetter],
            &[Ability::AscendantLight],
            // just enough space to do this
            &[Ability::Slide, Ability::SolarWind],
            &[Ability::ClingGem],
        ])],
    },
    // a speedrunner's one true love
    Check {
        location: Location::Prison,
        index: 356,
        drop: Drop::Ability(Ability::Slide),
        locks: &[],
    },
    // the little shit up in the rafters
    Check {
        location: Location::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves, Ability::Sunsetter],
            &[Ability::SunGreaves, Ability::Slide, Ability::SolarWind],
            &[Ability::ClingGem],
        ])],
    },
    // missable high-walled room next to key
    Check {
        location: Location::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves],
            &[Ability::ClingGem],
        ])],
    },
    // kill strong eyes, get the prize
    Check {
        location: Location::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: &[],
    },
    // need to document dark section argh
    // it's good enough for the final boss to divert the speedrun
    Check {
        location: Location::CastleSansa,
        index: 787,
        drop: Drop::Ability(Ability::Indignation),
        locks: &[],
    },
    // chillin by the window
    Check {
        location: Location::CastleSansa,
        index: 444,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[
            &[Ability::Sunsetter],
            &[Ability::SunGreaves],
            &[Ability::HeliacalPower],
            &[Ability::ClingGem],
            &[Ability::Slide, Ability::SolarWind],
        ])],
    },
    // idk what's professional about the outfit tbh
    Check {
        location: Location::CastleSansa,
        index: 791,
        drop: Drop::Ability(Ability::Professional),
        locks: &[Lock::SmallKey],
    },
    // just around the corner...
    Check {
        location: Location::CastleSansa,
        index: 495,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::ClingGem],
            &[Ability::SunGreaves, Ability::HeliacalPower],
        ])],
    },
    // has the extremely slappable wheel guys to help you
    Check {
        location: Location::CastleSansa,
        index: 494,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::AscendantLight],
            &[Ability::SunGreaves],
            &[Ability::ClingGem],
        ])],
    },
    // keeping you company in the softlock room
    Check {
        location: Location::CastleSansa,
        index: 498,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::ClingGem],
            &[
                Ability::Slide,
                Ability::SolarWind,
                Ability::SunGreaves,
                Ability::HeliacalPower,
            ],
        ])],
    },
    // through a tunnel over the haze and up
    Check {
        location: Location::CastleSansa,
        index: 443,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves],
            &[Ability::ClingGem],
        ])],
    },
    // just by the normal castle entrance
    Check {
        location: Location::CastleSansa,
        index: 497,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves],
            &[Ability::HeliacalPower],
            &[Ability::ClingGem],
        ])],
    },
    // taunting you between the two entrances
    Check {
        location: Location::CastleSansa,
        index: 496,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves, Ability::HeliacalPower],
            &[Ability::ClingGem],
            &[Ability::Slide, Ability::SolarWind, Ability::HeliacalPower],
            &[Ability::Slide, Ability::SolarWind, Ability::SunGreaves],
        ])],
    },
    // bouncing on the dude is really annoying
    Check {
        location: Location::CastleSansa,
        index: 788,
        drop: Drop::Ability(Ability::Pilgrimage),
        locks: &[Lock::Movement(&[
            &[Ability::AscendantLight],
            &[Ability::ClingGem],
            &[Ability::SunGreaves, Ability::HeliacalPower],
        ])],
    },
    // i always forget this room exists - has two doors and levers on each side
    Check {
        location: Location::CastleSansa,
        index: 268,
        drop: Drop::Ability(Ability::GoodGraces),
        locks: &[Lock::Movement(&[
            &[Ability::ClingGem],
            &[Ability::SunGreaves],
        ])],
    },
    // must not forget cool moon room
    // Listless Library
    // one of the most unique movement mechanics in a any videogame - i applaud you rittz
    Check {
        location: Location::MainLibrary,
        index: 267,
        drop: Drop::Ability(Ability::SunGreaves),
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves],
            &[Ability::HeliacalPower],
            &[Ability::ClingGem],
            &[Ability::Slide, Ability::SolarWind],
        ])],
    },
    // the room after the sun greaves demo zone
    Check {
        location: Location::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[&[Ability::SunGreaves]])],
    },
    // chillin in the hay
    Check {
        location: Location::Restricted,
        index: 268,
        drop: Drop::Ability(Ability::ClearMind),
        locks: &[Lock::Movement(&[
            &[Ability::Slide, Ability::SolarWind],
            &[Ability::SunGreaves],
            &[Ability::HeliacalPower],
            &[Ability::ClingGem],
        ])],
    },
    // tucked deep in an alcove
    Check {
        location: Location::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[Ability::SunGreaves],
            &[Ability::ClingGem],
        ])],
    },
    // Empty Bailey
    // in the building you slide into
    Check {
        location: Location::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[&[Ability::Slide]])],
    },
    // the hand is where you want to stand >:/
    Check {
        location: Location::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: &[Lock::Movement(&[
            &[Ability::Sunsetter],
            &[Ability::SunGreaves],
        ])],
    },
    // the airy jump feels so schmoovey
    Check {
        location: Location::EmptyBailey,
        index: 104,
        drop: Drop::Ability(Ability::SolarWind),
        locks: &[Lock::Movement(&[&[Ability::Slide]])],
    },
];
