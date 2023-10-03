use super::*;

use Ability as A;
use Location as L;

pub const CHECKS: [Check; 49] = [
    // not listing dream breaker since there's nothing it could be replaced by
    // the first thing you see
    Check {
        location: L::Prison,
        index: 214,
        drop: Drop::Health,
        locks: &[
            &[Lock::Movement(&[
                &[A::Sunsetter],
                &[A::AscendantLight],
                // just enough space to do this
                &[A::Slide, A::SolarWind],
                &[A::ClingGem],
            ])],
            // you can drop down from the entrance
            &[Lock::Location(Location::CastleSansa)],
        ],
    },
    // a speedrunner's one true love
    Check {
        location: L::Prison,
        index: 356,
        drop: Drop::Ability(A::Slide),
        locks: &[&[Lock::Movement(&[
            &[A::Slide],
            &[A::SunGreaves],
            &[A::Sunsetter, A::HeliacalPower],
            &[A::ClingGem],
        ])]],
    },
    Check {
        location: L::Prison,
        index: 357,
        drop: Drop::Ability(A::GoodGraces),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::AscendantLight, A::SunGreaves],
        ])]],
    },
    // the little shit up in the rafters
    Check {
        location: L::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves, A::Sunsetter],
            &[A::SunGreaves, A::Slide, A::SolarWind],
            &[A::ClingGem],
        ])]],
    },
    // missable high-walled room next to key
    Check {
        location: L::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // kill strong eyes, get the prize
    Check {
        location: L::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: &[],
    },
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
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    // idk what's professional about the outfit tbh
    Check {
        location: L::CastleSansa,
        index: 791,
        drop: Drop::Ability(A::Professional),
        locks: &[&[Lock::SmallKey]],
    },
    // just around the corner...
    Check {
        location: L::CastleSansa,
        index: 495,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    // has the extremely slappable wheel guys to help you
    Check {
        location: L::CastleSansa,
        index: 494,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])]],
    },
    // keeping you company in the softlock room
    Check {
        location: L::CastleSansa,
        index: 498,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    // must not forget cool moon room
    Check {
        location: L::CastleSansa,
        index: 790,
        drop: Drop::Ability(A::ClearMind),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    // through a tunnel over the haze and up
    Check {
        location: L::CastleSansa,
        index: 443,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // just by the normal castle entrance
    Check {
        location: L::CastleSansa,
        index: 497,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
        ])]],
    },
    // taunting you between the two entrances
    Check {
        location: L::CastleSansa,
        index: 496,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::HeliacalPower],
            &[A::Slide, A::SolarWind, A::SunGreaves],
        ])]],
    },
    // bouncing on the dude is really annoying
    Check {
        location: L::CastleSansa,
        index: 788,
        drop: Drop::Ability(A::Pilgrimage),
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight, A::Sunsetter],
            &[A::AscendantLight, A::SunGreaves],
            &[A::AscendantLight, A::SunGreaves],
            &[A::AscendantLight, A::Slide, A::SolarWind],
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem, A::HeliacalPower],
            &[A::ClingGem, A::Sunsetter],
            &[A::ClingGem, A::Slide, A::SolarWind],
        ])]],
    },
    // i always forget this room exists - has two doors and levers on each side
    Check {
        location: L::CastleSansa,
        index: 789,
        drop: Drop::Ability(A::GoodGraces),
        locks: &[&[Lock::Movement(&[&[A::ClingGem], &[A::SunGreaves]])]],
    },
    // Listless Library
    // one of the most unique movement mechanics in a any videogame - i applaud you rittz
    Check {
        location: L::MainLibrary,
        index: 267,
        drop: Drop::Ability(A::SunGreaves),
        locks: &[&[Lock::Movement(&[
            &[A::Slide, A::SunGreaves],
            &[A::Slide, A::HeliacalPower],
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    // the room after the sun greaves demo zone
    Check {
        location: L::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // chillin in the hay
    Check {
        location: L::Restricted,
        index: 268,
        drop: Drop::Ability(A::ClearMind),
        locks: &[&[Lock::Movement(&[
            &[A::Slide, A::SolarWind],
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
        ])]],
    },
    // tucked deep in an alcove
    Check {
        location: L::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // Sansa Keep
    // rittz is probably really annoyed people keep saying there's a missing texture
    Check {
        location: L::SansaKeep,
        index: 393,
        drop: Drop::Ability(A::Strikebreak),
        locks: &[&[Lock::Movement(&[
            &[A::Strikebreak, A::Slide, A::SolarWind],
            &[A::Strikebreak, A::SunGreaves],
            &[A::Strikebreak, A::ClingGem],
            // this is if you come from a certain entrance which is free
            &[A::Strikebreak, A::HeliacalPower],
        ])]],
    },
    // backflip is honestly the best combo finisher
    Check {
        location: L::Sunsetter,
        index: 392,
        drop: Drop::Ability(A::Sunsetter),
        locks: &[],
    },
    // this key is entirely pointless
    Check {
        location: L::Sunsetter,
        index: 251,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    // the key with a lever on each side
    Check {
        location: L::SansaKeep,
        index: 226,
        drop: Drop::SmallKey,
        locks: &[],
    },
    // tucked near the theatre entrance
    Check {
        location: L::SansaKeep,
        index: 394,
        drop: Drop::Ability(A::ClearMind),
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::HeliacalPower],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])]],
    },
    Check {
        location: L::SansaKeep,
        index: 227,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight, A::ClingGem, A::Sunsetter],
            &[A::AscendantLight, A::ClingGem, A::SunGreaves],
            &[
                A::Slide,
                A::SolarWind,
                A::Sunsetter,
                A::ClingGem,
                A::SunGreaves,
            ],
        ])]],
    },
    // Empty Bailey
    // in the building you slide into
    Check {
        location: L::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[&[A::Slide]])]],
    },
    // the hand is where you want to stand >:/
    Check {
        location: L::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])]],
    },
    // the airy jump feels so schmoovey
    Check {
        location: L::EmptyBailey,
        index: 104,
        drop: Drop::Ability(A::SolarWind),
        locks: &[&[Lock::Movement(&[&[A::Slide]])]],
    },
    // on the building in the middle
    Check {
        location: L::EmptyBailey,
        index: 66,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter, A::Slide],
            &[A::SunGreaves],
        ])]],
    },
    // CHEESE BELL
    Check {
        location: L::EmptyBailey,
        index: 105,
        drop: Drop::Ability(A::Empathy),
        locks: &[&[Lock::Movement(&[
            &[A::Slide, A::SolarWind, A::Sunsetter, A::HeliacalPower],
            &[A::Sunsetter, A::SunGreaves],
            &[A::Slide, A::SolarWind, A::ClingGem],
        ])]],
    },
    // Underbelly
    // hiding in the top right corner
    Check {
        location: L::Hole,
        index: 515,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::Sunsetter, A::HeliacalPower],
        ])]],
    },
    // literally the only other time soul cutter is used and speedrunners skipped it
    Check {
        location: L::Hole,
        index: 446,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter, A::SoulCutter, A::AscendantLight],
            &[A::Sunsetter, A::SunGreaves, A::Slide, A::SolarWind],
            &[A::Sunsetter, A::SoulCutter, A::ClingGem],
        ])]],
    },
    // aw hell yea i love smacking holes in the fabric of reality
    Check {
        location: L::MainUnderbelly,
        index: 834,
        drop: Drop::Ability(A::AscendantLight),
        // you can go through the dark area
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight],
            &[A::SunGreaves],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::HeliacalPower],
        ])]],
    },
    // in an alcove behind some pillars
    Check {
        location: L::MainUnderbelly,
        index: 517,
        drop: Drop::Health,
        locks: &[],
    },
    // a lot of people missed this one
    Check {
        location: L::MainUnderbelly,
        index: 447,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    // really easy strikebreak nook to miss
    Check {
        location: L::MainUnderbelly,
        index: 835,
        drop: Drop::Ability(A::MartialProwess),
        locks: &[&[Lock::Movement(&[
            &[A::Strikebreak, A::AscendantLight, A::HeliacalPower],
            &[A::Strikebreak, A::AscendantLight, A::SunGreaves],
            &[A::Strikebreak, A::AscendantLight, A::Sunsetter],
            &[A::Strikebreak, A::AscendantLight, A::Slide, A::SolarWind],
        ])]],
    },
    // i find it cool you can get this before sun greaves
    Check {
        location: L::MainUnderbelly,
        index: 836,
        drop: Drop::Ability(A::HeliacalPower),
        locks: &[&[
            Lock::SmallKey,
            Lock::Movement(&[&[A::Slide, A::SunGreaves], &[A::Slide, A::Sunsetter]]),
        ]],
    },
    // on top of the big building
    Check {
        location: L::MainUnderbelly,
        index: 516,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::Sunsetter]])]],
    },
    // Tower Ruins
    // the most broken ability in the game, in both ways
    Check {
        location: L::TowerRuins,
        index: 89,
        drop: Drop::Ability(A::ClingGem),
        locks: &[&[Lock::Movement(&[&[A::SunGreaves]])]],
    },
    // dunno why it's there when the ending door is also right where it is
    Check {
        location: L::TowerRuins,
        index: 56,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves, A::ClingGem]])]],
    },
    // on a beam in a corner
    Check {
        location: L::PillarRoom,
        index: 1080,
        drop: Drop::Ability(A::AerialFinesse),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem, A::SunGreaves],
            &[A::Slide, A::SolarWind, A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves],
        ])]],
    },
    // hiding amid the boxes like a murderous goatling
    Check {
        location: L::MainTheatre,
        index: 843,
        drop: Drop::Health,
        locks: &[],
    },
    // the most convoluted big key to get
    Check {
        location: L::MainTheatre,
        index: 802,
        drop: Drop::BigKey,
        // there's one gap in the open green room with enemies which is too big
        locks: &[&[Lock::Movement(&[
            &[
                A::Strikebreak,
                A::SoulCutter,
                A::ClingGem,
                A::Slide,
                A::SolarWind,
            ],
            &[A::Strikebreak, A::SoulCutter, A::ClingGem, A::HeliacalPower],
            &[A::Strikebreak, A::SoulCutter, A::ClingGem, A::SunGreaves],
        ])]],
    },
    // i'm so bad at aiming this lol
    Check {
        location: L::MainTheatre,
        index: 1079,
        drop: Drop::Ability(A::SoulCutter),
        locks: &[&[Lock::Movement(&[&[A::Strikebreak]])]],
    },
    // right in the centre on a pillar
    Check {
        location: L::MainTheatre,
        index: 844,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // next to the last check behind a locked door
    Check {
        location: L::MainTheatre,
        index: 1081,
        drop: Drop::Ability(A::Empathy),
        locks: &[&[
            Lock::SmallKey,
            Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]]),
        ]],
    },
];
