use super::*;

use Ability as A;
use Location as L;

pub const CHECKS: [Check] = [
    // not listing dream breaker since there's nothing it could be replaced by
    // the first thing you see
    Check {
        location: L::Prison,
        index: 214,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::AscendantLight],
            // just enough space to do this
            &[A::Slide, A::SolarWind],
            &[A::ClingGem],
        ])],
    },
    // a speedrunner's one true love
    Check {
        location: L::Prison,
        index: 356,
        drop: Drop::Ability(A::Slide),
        locks: &[],
    },
    // the little shit up in the rafters
    Check {
        location: L::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[
            &[A::SunGreaves, A::Sunsetter],
            &[A::SunGreaves, A::Slide, A::SolarWind],
            &[A::ClingGem],
        ])],
    },
    // missable high-walled room next to key
    Check {
        location: L::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])],
    },
    // kill strong eyes, get the prize
    Check {
        location: L::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: &[],
    },
    // need to document dark section argh
    // it's good enough for the final boss to divert the speedrun
    Check {
        location: L::CastleSansa,
        index: 787,
        drop: Drop::Ability(A::Indignation),
        locks: &[],
    },
    // chillin by the window
    Check {
        location: L::CastleSansa,
        index: 444,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])],
    },
    // idk what's professional about the outfit tbh
    Check {
        location: L::CastleSansa,
        index: 791,
        drop: Drop::Ability(A::Professional),
        locks: &[Lock::SmallKey],
    },
    // just around the corner...
    Check {
        location: L::CastleSansa,
        index: 495,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::ClingGem],
            &[A::SunGreaves, A::HeliacalPower],
        ])],
    },
    // has the extremely slappable wheel guys to help you
    Check {
        location: L::CastleSansa,
        index: 494,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::AscendantLight],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])],
    },
    // keeping you company in the softlock room
    Check {
        location: L::CastleSansa,
        index: 498,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])],
    },
    // must not forget cool moon room
    Check {
        location: L::CastleSansa,
        index: 789,
        drop: Drop::Ability(A::ClearMind),
        locks: &[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])],
    },
    // through a tunnel over the haze and up
    Check {
        location: L::CastleSansa,
        index: 443,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])],
    },
    // just by the normal castle entrance
    Check {
        location: L::CastleSansa,
        index: 497,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
        ])],
    },
    // taunting you between the two entrances
    Check {
        location: L::CastleSansa,
        index: 496,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::HeliacalPower],
            &[A::Slide, A::SolarWind, A::SunGreaves],
        ])],
    },
    // bouncing on the dude is really annoying
    Check {
        location: L::CastleSansa,
        index: 788,
        drop: Drop::Ability(A::Pilgrimage),
        locks: &[Lock::Movement(&[
            &[A::AscendantLight],
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem, A::HeliacalPower],
            &[A::ClingGem, A::Sunsetter],
            &[A::ClingGem, A::Slide, A::SolarWind],
        ])],
    },
    // i always forget this room exists - has two doors and levers on each side
    Check {
        location: L::CastleSansa,
        index: 268,
        drop: Drop::Ability(A::GoodGraces),
        locks: &[Lock::Movement(&[&[A::ClingGem], &[A::SunGreaves]])],
    },
    // Listless Library
    // one of the most unique movement mechanics in a any videogame - i applaud you rittz
    Check {
        location: L::MainLibrary,
        index: 267,
        drop: Drop::Ability(A::SunGreaves),
        locks: &[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])],
    },
    // the room after the sun greaves demo zone
    Check {
        location: L::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[&[A::SunGreaves]])],
    },
    // chillin in the hay
    Check {
        location: L::Restricted,
        index: 268,
        drop: Drop::Ability(A::ClearMind),
        locks: &[Lock::Movement(&[
            &[A::Slide, A::SolarWind],
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
        ])],
    },
    // tucked deep in an alcove
    Check {
        location: L::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: &[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])],
    },
    // Sansa Keep
    Check {
        location: L::SansaKeep,
        index: 393,
        drop: Drop::Ability(A::Strikebreak),
        locks: &[Lock::Movement(&[
            &[A::Slide, A::Strikebreak, A::SolarWind],
            &[A::Slide, A::Strikebreak, A::SunGreaves],
            &[A::Slide, A::Strikebreak, A::ClingGem],
            // this is if you come from a certain entrance which is free
            &[A::Slide, A::Strikebreak, A::HeliacalPower],
        ])],
    },
    Check {
        location: L::SansaKeep,
        index: 392,
        drop: Drop::Ability(A::Sunsetter),
        // either get over the wall with parkour or use key so need to refactor locks lol
        locks: &[Lock::Movement(&[&[A::Sunsetter], todo!()])],
    },
    Check {
        location: L::SansaKeep,
        index: 251,
        drop: Drop::Health,
        // uses same parkour or key rule but also with height stuff
        locks: &[Lock::Movement(&[&[todo!()]])],
    },
    Check {
        location: L::SansaKeep,
        index: 226,
        drop: Drop::SmallKey,
        locks: &[],
    },
    Check {
        location: L::SansaKeep,
        index: 394,
        drop: Drop::Ability(A::ClearMind),
        locks: &[Lock::Movement(&[
            &[Ability::Sunsetter],
            &[Ability::HeliacalPower],
            &[Ability::SunGreaves],
            &[Ability::ClingGem],
        ])],
    },
    // need to do the big key
    // Empty Bailey
    // in the building you slide into
    Check {
        location: L::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: &[Lock::Movement(&[&[A::Slide]])],
    },
    // the hand is where you want to stand >:/
    Check {
        location: L::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: &[Lock::Movement(&[&[A::Sunsetter], &[A::SunGreaves]])],
    },
    // the airy jump feels so schmoovey
    Check {
        location: L::EmptyBailey,
        index: 104,
        drop: Drop::Ability(A::SolarWind),
        locks: &[Lock::Movement(&[&[A::Slide]])],
    },
    // Tower ruins
    Check {
        location: L::TowerRuins,
        index: 89,
        drop: Drop::Ability(A::ClingGem),
        locks: &[Lock::Movement(&[&[A::SunGreaves]])],
    },
    Check {
        location: L::TowerRuins,
        index: 56,
        drop: Drop::BigKey,
        locks: &[Lock::Movement(&[&[A::SunGreaves, A::ClingGem]])],
    },
];
