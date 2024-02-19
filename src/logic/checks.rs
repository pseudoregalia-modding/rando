use super::*;

use Ability as A;
use Location as L;

pub const CHECKS: [Check; 81] = [
    // dream breaker is randomised with random start
    Check {
        description: "where dream breaker normally is",
        location: L::Prison,
        index: 355,
        drop: Drop::Ability(A::DreamBreaker),
        locks: &[],
    },
    Check {
        description: "where the first health piece is",
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
                &[A::SunGreaves],
                &[A::HeliacalPower],
            ])],
            // you can drop down from the entrance
            &[Lock::Location(Location::CastleSansa)],
        ],
    },
    Check {
        description: "where slide normally is",
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
        description: "black hole parkour off the beaten path",
        location: L::Prison,
        index: 357,
        drop: Drop::Ability(A::GoodGraces),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::AscendantLight, A::SunGreaves],
        ])]],
    },
    Check {
        description: "up in the rafters",
        location: L::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves, A::Sunsetter],
            &[A::SunGreaves, A::Slide, A::SolarWind],
            &[A::ClingGem],
        ])]],
    },
    Check {
        description: "missable high walled room",
        location: L::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    Check {
        description: "strong eyes' lair",
        location: L::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: &[],
    },
    Check {
        description: "the goatling who wants to lick the checkpoint",
        location: L::CastleSansa,
        index: 631,
        drop: Drop::Goatling(&[
            "These [#7c79e8](crystals) are pretty nice, right?",
            "They make me feel safe...",
            "I think i'm gonna lick it. I bet it's full of [#8ada1c, buoy, italics](minerals).",
        ]),
        locks: &[],
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CastleSansa,
        index: 634,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CastleSansa,
        index: 635,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CastleSansa,
        index: 636,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "where indignation normally is",
        location: L::CastleSansa,
        index: 787,
        drop: Drop::Ability(A::Indignation),
        locks: &[],
    },
    Check {
        description: "chillin' on a ledge by the window",
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
    Check {
        description: "where the professional normally is",
        location: L::CastleSansa,
        index: 791,
        drop: Drop::Ability(A::Professional),
        locks: &[&[Lock::SmallKey]],
    },
    Check {
        description: "tucked deep in a corner in the bouncer room",
        location: L::CastleSansa,
        index: 495,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    Check {
        description: "the extremely slappable wheel guy room",
        location: L::CastleSansa,
        index: 494,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])]],
    },
    Check {
        description: "the old softlock room",
        location: L::CastleSansa,
        index: 498,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    Check {
        description: "the goatling about to jump into the haze",
        location: L::CastleSansa,
        index: 633,
        drop: Drop::Goatling(&[
            "Oh, thanks for breaking that wall down.",
            "Thought I was gonna have to jump into the haze..."
        ]),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    Check {
        description: "cool moon room",
        location: L::CastleSansa,
        index: 790,
        drop: Drop::Ability(A::ClearMind),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves, A::HeliacalPower],
        ])]],
    },
    Check {
        description: "through the wallkick tunnel",
        location: L::CastleSansa,
        index: 443,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    Check {
        description: "in the pit next to the dungeon entrance",
        location: L::CastleSansa,
        index: 497,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
        ])]],
    },
    Check {
        description: "the goatling that calls you bubble girl",
        location: L::CastleSansa,
        index: 630,
        drop: Drop::Goatling(&[
            "I was supposed to go help in the [#cf2525](theatre), but I can't really get through here.",
            "I just don't really wanna touch the [#cf2525](bubbles)...",
            "What? I dont have a problem. You go touch 'em then, bubble girl."
        ]),
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SunGreaves]
        ])]],
    },
    Check {
        description: "on the ledge above the bailey entrance",
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
    Check {
        description: "the goatling in the gazebo",
        location: L::CastleSansa,
        index: 632,
        drop: Drop::Goatling(&[
            "The princess used to love having afternoon tea here.",
            "But the handmaiden has run out of her special ingredient.",
            "I guess the princess doesn't really want anybody else's tea...",
        ]),
        locks: &[],
    },
    Check {
        description: "the chair in the gazebo",
        location: L::CastleSansa,
        index: 637,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "next to a bouncer in the massive room",
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
    Check {
        description: "in the room with two other ones to each side",
        location: L::CastleSansa,
        index: 789,
        drop: Drop::Ability(A::GoodGraces),
        locks: &[&[Lock::Movement(&[&[A::ClingGem], &[A::SunGreaves]])]],
    },
    // Listless Library
    Check {
        description: "the chair at the entrance",
        location: L::MainLibrary,
        index: 241,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "where sun greaves normally are",
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
    Check {
        description: "the chair after the normal sun greaves location",
        location: L::MainLibrary,
        index: 243,
        drop: Drop::Chair,
        locks: &[&[Lock::Movement(&[
            &[A::Slide, A::SunGreaves],
            &[A::Slide, A::HeliacalPower],
            &[A::SunGreaves, A::HeliacalPower],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "the note next to the egg nest",
        location: L::MainLibrary,
        index: 240,
        drop: Drop::Note,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "the chair next to the egg nest",
        location: L::MainLibrary,
        index: 242,
        drop: Drop::Chair,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "in the buttress room",
        location: L::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    Check {
        description: "in the hay behind the locked door",
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
    Check {
        description: "tucked deep behind the locked door",
        location: L::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    // Sansa Keep
    Check {
        description: "the goatling sad about the lack of furniture",
        location: L::SansaKeep,
        index: 323,
        drop: Drop::Goatling(&[
            "They took away all my furniture."
        ]),
        locks: &[],
    },
    Check {
        description: "the goatling collapsing out of reality",
        location: L::SansaKeep,
        index: 324,
        drop: Drop::Goatling(&[
            "[6rr](.....c.....y..u...i....y.......ce....)"
        ]),
        locks: &[],
    },
    Check {
        description: "the chair collapsing out of reality",
        location: L::SansaKeep,
        index: 325,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "where strikebreak normally is",
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
    Check {
        description: "where sunsetter normally is",
        location: L::Sunsetter,
        index: 392,
        drop: Drop::Ability(A::Sunsetter),
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::HeliacalPower],
            &[A::SunGreaves],
            &[A::ClingGem],
        ])]],
    },
    Check {
        description: "in an alcove next to the locked door",
        location: L::Sunsetter,
        index: 251,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "in the room with a lever on each side",
        location: L::SansaKeep,
        index: 226,
        drop: Drop::SmallKey,
        locks: &[],
    },
    Check {
        description: "tucked near the theatre entrance",
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
        description: "at the end of the parkour",
        location: L::SansaKeep,
        index: 227,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight, A::ClingGem, A::Sunsetter],
            &[A::AscendantLight, A::ClingGem, A::SunGreaves],
            &[A::AscendantLight, A::Sunsetter, A::SunGreaves],
            &[
                A::Slide,
                A::SolarWind,
                A::Sunsetter,
                A::ClingGem,
                A::SunGreaves,
            ],
        ])]],
    },
    Check {
        description: "the chair in the middle of the parkour",
        location: L::SansaKeep,
        index: 326,
        drop: Drop::Chair,
        locks: &[&[Lock::Movement(&[
            &[A::AscendantLight, A::ClingGem, A::Sunsetter],
            &[A::AscendantLight, A::ClingGem, A::SunGreaves],
            &[A::AscendantLight, A::Sunsetter, A::SunGreaves],
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
    Check {
        description: "the goatling who's hiding",
        location: L::EmptyBailey,
        index: 88,
        drop: Drop::Goatling(&["...i'm not here."]),
        locks: &[&[Lock::Movement(&[&[A::Slide]])]],
    },
    Check {
        description: "in the building you slide into",
        location: L::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[&[A::Slide]])]],
    },
    Check {
        description: "guarded by the hand and soldier",
        location: L::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::SunGreaves],
            &[A::ClingGem],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "where solar wind normally is",
        location: L::EmptyBailey,
        index: 104,
        drop: Drop::Ability(A::SolarWind),
        locks: &[&[Lock::Movement(&[
            // need some way to cross the gap afterwards
            &[A::Slide, A::SolarWind],
            &[A::Slide, A::HeliacalPower],
            &[A::Slide, A::SunGreaves],
        ])]],
    },
    Check {
        description: "in the tower in the middle",
        location: L::EmptyBailey,
        index: 66,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter, A::HeliacalPower],
            &[A::SunGreaves],
            // you can jump down from cheese bell
            &[A::Slide, A::SolarWind, A::ClingGem],
        ])]],
    },
    Check {
        description: "under the cheese bell",
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
    Check {
        description: "near the entrance from sansa keep",
        location: L::Hole,
        index: 515,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::Sunsetter, A::HeliacalPower],
        ])]],
    },
    Check {
        description: "the soul cutter lever room",
        location: L::Hole,
        index: 446,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter, A::SoulCutter, A::AscendantLight],
            &[A::Sunsetter, A::SunGreaves, A::Slide, A::SolarWind],
            &[A::Sunsetter, A::SoulCutter, A::ClingGem],
        ])]],
    },
    Check {
        description: "where ascendant light normally is",
        location: L::MainUnderbelly,
        index: 834,
        drop: Drop::Ability(A::AscendantLight),
        // you can go through the dark area and there's a passage which you can do with nothing
        locks: &[
            // &[Lock::Movement(&[
            // &[A::AscendantLight],
            // &[A::SunGreaves],
            // &[A::ClingGem],
            // &[A::Slide, A::SolarWind, A::HeliacalPower],
            // ])]
        ],
    },
    Check {
        description: "in an alcove behind some pillars",
        location: L::MainUnderbelly,
        index: 517,
        drop: Drop::Health,
        locks: &[],
    },
    Check {
        description: "on a missable ledge in the centre",
        location: L::MainUnderbelly,
        index: 447,
        drop: Drop::SmallKey,
        locks: &[&[Lock::Movement(&[
            &[A::Sunsetter],
            &[A::Slide, A::SolarWind],
        ])]],
    },
    Check {
        description: "the note on a high ledge in the big room",
        location: L::MainUnderbelly,
        index: 702,
        drop: Drop::Note,
        locks: &[&[Lock::Movement(&[
            &[A::Slide, A::SolarWind, A::HeliacalPower, A::Sunsetter],
            &[A::ClingGem, A::HeliacalPower, A::Sunsetter],
            &[A::SunGreaves, A::HeliacalPower, A::Sunsetter]
        ])]],
    },
    Check {
        description: "black hole parkour behind strikebreak wall",
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
    Check {
        description: "behind the locked door",
        location: L::MainUnderbelly,
        index: 836,
        drop: Drop::Ability(A::HeliacalPower),
        locks: &[&[
            Lock::SmallKey,
            Lock::Movement(&[&[A::Slide, A::SunGreaves], &[A::Slide, A::Sunsetter]]),
        ]],
    },
    Check {
        description: "the note behind the locked door",
        location: L::MainUnderbelly,
        index: 703,
        drop: Drop::Note,
        locks: &[&[
            Lock::SmallKey,
            Lock::Movement(&[&[A::Slide, A::SunGreaves], &[A::Slide, A::Sunsetter]]),
        ]],
    },
    Check {
        description: "the note near the empty bailey entrance",
        location: L::MainUnderbelly,
        index: 704,
        drop: Drop::Note,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves],
            &[A::HeliacalPower, A::Sunsetter],
            &[A::ClingGem],
            &[A::AscendantLight],
            &[A::Slide, A::SolarWind]
        ])]],
    },
    Check {
        description: "on top of the big building",
        location: L::MainUnderbelly,
        index: 516,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::Sunsetter]])]],
    },
    // Tower Ruins
    Check {
        description: "where cling gem normally is",
        location: L::TowerRuins,
        index: 89,
        drop: Drop::Ability(A::ClingGem),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem],
            &[A::SunGreaves],
            &[A::HeliacalPower, A::Sunsetter],
        ])]],
    },
    Check {
        description: "atop the tower",
        location: L::TowerRuins,
        index: 56,
        drop: Drop::BigKey,
        locks: &[&[Lock::Movement(&[
            &[A::SunGreaves, A::ClingGem],
            &[A::HeliacalPower, A::Sunsetter, A::ClingGem],
        ])]],
    },
    // Twilight Theatre
    Check {
        description: "on a beam in the corner",
        location: L::PillarRoom,
        index: 1080,
        drop: Drop::Ability(A::AerialFinesse),
        locks: &[&[Lock::Movement(&[
            &[A::ClingGem, A::SunGreaves],
            &[A::Slide, A::SolarWind, A::ClingGem],
            &[A::Slide, A::SolarWind, A::SunGreaves],
        ])]],
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 949,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 950,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 946,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "the chair next to the books",
        location: L::OtherTheatrePath,
        index: 951,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "the goatling who can eat 20 beans at least",
        location: L::TheatreEntrance,
        index: 937,
        drop: Drop::Goatling(&[
            "where have [#cf2525](you been)?",
            "three bean casserole? not enough for 1 man. i can eat like, [#cf2525](20 beans), at least.",
            "so get to it. [up, 10rr](please?)"
        ]),
        locks: &[],
    },
    Check {
        description: "the goatling who thought the theatre was safe",
        location: L::TheatreEntrance,
        index: 939,
        drop: Drop::Goatling(&[
            "I heard that the theatre was still in good condition...",
            "But it seems even this place has been affected."
        ]),
        locks: &[],
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 944,
        drop: Drop::Goatling(&[
            "Ah nuts....I really wanted to see the show today."
        ]),
        locks: &[],
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 945,
        drop: Drop::Goatling(&[
            "Sorry miss, can't let you in.",
            "Theatre's closed until all the haze is gone."
        ]),
        locks: &[],
    },
    Check {
        description: "hiding amid the boxes",
        location: L::MainTheatre,
        index: 843,
        drop: Drop::Health,
        locks: &[],
    },
    Check {
        description: "the goatling that will kill again",
        location: L::MainTheatre,
        index: 936,
        drop: Drop::Goatling(&[
            "please leave me alone?",
            "i will [#cf2525](kill again)"
        ]),
        locks: &[&[
            Lock::Movement(&[
                &[A::Slide, A::SolarWind, A::HeliacalPower],
                &[A::SunGreaves],
                &[A::ClingGem]
            ])
        ]],
    },
    Check {
        description: "the chair near the courtyard",
        location: L::MainTheatre,
        index: 947,
        drop: Drop::Chair,
        locks: &[],
    },
    Check {
        description: "the chair in the soul cutter zone",
        location: L::MainTheatre,
        index: 948,
        drop: Drop::Chair,
        locks: &[&[Lock::Movement(&[
            &[A::Strikebreak, A::SoulCutter, A::SunGreaves],
            &[A::Strikebreak, A::SoulCutter, A::ClingGem],
        ])]],
    },
    Check {
        description: "behind three maximum security cages",
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
    Check {
        description: "where soul cutter normally is",
        location: L::MainTheatre,
        index: 1079,
        drop: Drop::Ability(A::SoulCutter),
        locks: &[&[Lock::Movement(&[
            &[A::Strikebreak, A::SoulCutter, A::ClingGem],
            &[A::Strikebreak, A::SoulCutter, A::SunGreaves],
        ])]],
    },
    Check {
        description: "in the back on a pillar",
        location: L::MainTheatre,
        index: 844,
        drop: Drop::Health,
        locks: &[&[Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]])]],
    },
    Check {
        description: "behind the locked door",
        location: L::MainTheatre,
        index: 1081,
        drop: Drop::Ability(A::Empathy),
        locks: &[&[
            Lock::SmallKey,
            Lock::Movement(&[&[A::SunGreaves], &[A::ClingGem]]),
        ]],
    },
];
