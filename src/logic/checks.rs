use super::*;

use Ability as A;
use Location as L;
use Lock::{All, Any, Movement as Powerup, Location as Loc};

pub const CHECKS: [Check; 81] = [
    // dream breaker is randomised with random start
    Check {
        description: "where dream breaker normally is",
        location: L::VDreamBreaker,
        index: 355,
        drop: Drop::Ability(A::DreamBreaker),
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "where the first health piece is",
        location: L::EarlyPrison,
        index: 214,
        drop: Drop::Health,
        locks: Any(&[
            Any(&[
                Powerup(A::Sunsetter),
                Powerup(A::AscendantLight),
                // just enough space to do this
                Powerup(A::SolarWind),
                Powerup(A::ClingGem(4)),
                Powerup(A::HeliacalPower),
            ]),
            // you can drop down from the entrance
            Loc(Location::CsMain),
        ]),
    },
    Check {
        description: "where slide normally is",
        location: L::LatePrison,
        index: 356,
        drop: Drop::Ability(A::Slide),
        locks: Any(&[
            Powerup(A::Slide),
            Powerup(A::SunGreaves),
            All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
            Powerup(A::ClingGem(2)),
        ]),
    },
    Check {
        description: "black hole parkour off the beaten path",
        location: L::LatePrison,
        index: 357,
        drop: Drop::Ability(A::GoodGraces),
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::AscendantLight), Powerup(A::SunGreaves)]),
        ]),
    },
    Check {
        description: "up in the rafters",
        location: L::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: Any(&[
            All(&[
                Powerup(A::SunGreaves),
                Any(&[
                    Powerup(A::Sunsetter),
                    Powerup(A::SolarWind)
                ]),
            ]),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "missable high walled room",
        location: L::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(4))]),
    },
    Check {
        description: "strong eyes' lair",
        location: L::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 631,
        drop: Drop::Goatling(&[
            "These [#7c79e8](crystals) are pretty nice, right?",
            "They make me feel safe...",
            "I think i'm gonna lick it. I bet it's full of [#8ada1c, buoy, italics](minerals).",
        ]),
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 634,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 635,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 636,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "where indignation normally is",
        location: L::CsMain,
        index: 787,
        drop: Drop::Ability(A::Indignation),
        locks: Lock::None,
    },
    Check {
        description: "chillin' on a ledge by the window",
        location: L::CsMain,
        index: 444,
        drop: Drop::SmallKey,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(2)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "where the professional normally is",
        location: L::CsMain,
        index: 791,
        drop: Drop::Ability(A::Professional),
        locks: All(&[
            Lock::SmallKey,
            Powerup(A::DreamBreaker)
        ]),
    },
    Check {
        description: "tucked deep in a corner in the bouncer room",
        location: L::CsMain,
        index: 495,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the extremely slappable wheel guy room",
        location: L::CsMain,
        index: 494,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::AscendantLight),
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(2)),
        ]),
    },
    Check {
        description: "the old softlock room",
        location: L::CsOldSoftlockRoom,
        index: 498,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the goatling about to jump into the haze",
        location: L::CsOldSoftlockRoom,
        index: 633,
        drop: Drop::Goatling(&[
            "Oh, thanks for breaking that wall down.",
            "Thought I was gonna have to jump into the haze..."
        ]),
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "cool moon room",
        location: L::CsTheatreEntrance,
        index: 790,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "through the wallkick tunnel",
        location: L::CsMain,
        index: 443,
        drop: Drop::SmallKey,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(6))]),
    },
    Check {
        description: "in the pit next to the dungeon entrance",
        location: L::CsMain,
        index: 497,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(6)),
        ]),
    },
    Check {
        description: "the goatling that calls you bubble girl",
        location: L::CsMain,
        index: 630,
        drop: Drop::Goatling(&[
            "I was supposed to go help in the [#cf2525](theatre), but I can't really get through here.",
            "I just don't really wanna touch the [#cf2525](bubbles)...",
            "What? I dont have a problem. You go touch 'em then, bubble girl."
        ]),
        locks: Any(&[
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(4)),
            All(&[Powerup(A::Slide), Powerup(A::SunGreaves)])
        ]),
    },
    Check {
        description: "on the ledge above the bailey entrance",
        location: L::CsMain,
        index: 496,
        drop: Drop::Health,
        locks: Any(&[
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
            Powerup(A::ClingGem(4)),
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the goatling in the gazebo",
        location: L::CsMain,
        index: 632,
        drop: Drop::Goatling(&[
            "The princess used to love having afternoon tea here.",
            "But the handmaiden has run out of her special ingredient.",
            "I guess the princess doesn't really want anybody else's tea...",
        ]),
        locks: Lock::None,
    },
    Check {
        description: "the chair in the gazebo",
        location: L::CsMain,
        index: 637,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "next to a bouncer in the massive room",
        location: L::CsMain,
        index: 788,
        drop: Drop::Ability(A::Pilgrimage),
        locks: Any(&[
            All(&[
                Powerup(A::AscendantLight),
                Any(&[
                    Powerup(A::Sunsetter),
                    Powerup(A::SunGreaves),
                    Powerup(A::SolarWind),
                ]),
            ]),
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
            All(&[
                Powerup(A::ClingGem(6)), 
                Any(&[
                    Powerup(A::HeliacalPower),
                    Powerup(A::Sunsetter),
                    Powerup(A::SolarWind),
                ]),
            ]),
        ]),
    },
    Check {
        description: "in the room with two other ones to each side",
        location: L::CsMain,
        index: 789,
        drop: Drop::Ability(A::GoodGraces),
        locks: Any(&[Powerup(A::ClingGem(6)), Powerup(A::SunGreaves)]),
    },
    // Listless Library
    Check {
        description: "the chair at the entrance",
        location: L::MainLibrary,
        index: 241,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "where sun greaves normally are",
        location: L::LibSaveNearGreaves,
        index: 267,
        drop: Drop::Ability(A::SunGreaves),
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "the chair after the normal sun greaves location",
        location: L::LibSaveNearGreaves,
        index: 243,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "the note next to the egg nest",
        location: L::MainLibrary,
        index: 240,
        drop: Drop::Note,
        locks: Any(&[
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "the chair next to the egg nest",
        location: L::MainLibrary,
        index: 242,
        drop: Drop::Chair,
        locks: Any(&[
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "in the buttress room",
        location: L::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(4))]),
    },
    Check {
        description: "in the hay behind the locked door",
        location: L::Restricted,
        index: 268,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            Powerup(A::SolarWind),
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "tucked deep behind the locked door",
        location: L::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(4))]),
    },
    // Sansa Keep
    Check {
        description: "the goatling sad about the lack of furniture",
        location: L::SansaKeep,
        index: 323,
        drop: Drop::Goatling(&[
            "They took away all my furniture."
        ]),
        locks: Lock::None,
    },
    Check {
        description: "the goatling collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 324,
        drop: Drop::Goatling(&[
            "[6rr](.....c.....y..u...i....y.......ce....)"
        ]),
        locks: Lock::None,
    },
    Check {
        description: "the chair collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 325,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "where strikebreak normally is",
        location: L::SansaKeep,
        index: 393,
        drop: Drop::Ability(A::Strikebreak),
        locks:  All(&[
            Powerup(A::Strikebreak),
            Any(&[
                Powerup(A::HeliacalPower),
                Powerup(A::ClingGem(4)),
                Powerup(A::SolarWind),
            ]),
        ]),
    },
    Check {
        description: "where sunsetter normally is",
        location: L::Sunsetter,
        index: 392,
        drop: Drop::Ability(A::Sunsetter),
        locks: All(&[
            Powerup(A::DreamBreaker),
            Any(&[
                Powerup(A::Sunsetter),
                Powerup(A::HeliacalPower),
                Powerup(A::ClingGem(2))
            ]),
        ]),
    },
    Check {
        description: "in an alcove next to the locked door",
        location: L::Sunsetter,
        index: 251,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::SunGreaves),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "in the room with a lever on each side",
        location: L::SansaKeep,
        index: 226,
        drop: Drop::SmallKey,
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "tucked near the theatre entrance",
        location: L::SansaKeep,
        index: 394,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "at the end of the parkour",
        location: L::SansaKeep,
        index: 227,
        drop: Drop::BigKey,
        locks: Any(&[
            All(&[
                Powerup(A::AscendantLight),
                Any(&[
                    All(&[
                        Powerup(A::ClingGem(4)),
                        Any(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                    ]),
                    All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                ]),
            ]),
            All(&[
                Powerup(A::DreamBreaker),
                Powerup(A::Slide),
                Powerup(A::SolarWind),
                Powerup(A::Sunsetter),
                Powerup(A::ClingGem(2)),
                Powerup(A::SunGreaves),
            ]),
        ]),
    },
    Check {
        description: "the chair in the middle of the parkour",
        location: L::SansaKeep,
        index: 326,
        drop: Drop::Chair,
        locks:  Any(&[
            All(&[
                Powerup(A::AscendantLight),
                Any(&[
                    All(&[
                        Powerup(A::ClingGem(4)),
                        Any(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                    ]),
                    All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                ]),
            ]),
            All(&[
                Powerup(A::DreamBreaker),
                Powerup(A::Slide),
                Powerup(A::SolarWind),
                Powerup(A::Sunsetter),
                Powerup(A::ClingGem(2)),
                Powerup(A::SunGreaves),
            ]),
        ]),
    },
    // Empty Bailey
    Check {
        description: "the goatling who's hiding",
        location: L::EmptyBailey,
        index: 88,
        drop: Drop::Goatling(&["...i'm not here."]),
        locks: Powerup(A::Slide),
    },
    Check {
        description: "in the building you slide into",
        location: L::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: Powerup(A::Slide),
    },
    Check {
        description: "guarded by the hand and soldier",
        location: L::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "where solar wind normally is",
        location: L::EmptyBailey,
        index: 104,
        drop: Drop::Ability(A::SolarWind),
        locks: All(&[
            Powerup(A::Slide),
            Powerup(A::DreamBreaker),
            Any(&[
                Powerup(A::SolarWind),
                Powerup(A::HeliacalPower),
            ]),
        ]),
    },
    Check {
        description: "in the tower in the middle",
        location: L::EmptyBailey,
        index: 66,
        drop: Drop::Health,
        locks: Any(&[
            All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
            Powerup(A::SunGreaves),
            // you can jump down from cheese bell
            All(&[Powerup(A::SolarWind), Powerup(A::ClingGem(6))]),
        ]),
    },
    Check {
        description: "under the cheese bell",
        location: L::EmptyBailey,
        index: 105,
        drop: Drop::Ability(A::Empathy),
        locks: Any(&[
            All(&[
                Powerup(A::SolarWind),
                Any(&[
                    Powerup(A::ClingGem(6)),
                    All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
                ]),
            ]),
            All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
        ]),
    },
    // Underbelly
    Check {
        description: "near the entrance from sansa keep",
        location: L::SansaHole,
        index: 515,
        drop: Drop::Health,
        locks: Any(&[
            Powerup(A::SunGreaves),
            All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the soul cutter lever room",
        location: L::SansaHole,
        index: 446,
        drop: Drop::BigKey,
        locks: All(&[
            Powerup(A::DreamBreaker),
            Powerup(A::Sunsetter),
            Any(&[
                All(&[
                    Powerup(A::SoulCutter),
                    Any(&[Powerup(A::AscendantLight), Powerup(A::ClingGem(6))]),
                ]),
                All(&[Powerup(A::SunGreaves), Powerup(A::SolarWind)]),
            ]),
        ]),
    },
    Check {
        description: "where ascendant light normally is",
        location: L::VAscendantLight,
        index: 834,
        drop: Drop::Ability(A::AscendantLight),
        // you can go through the dark area and there's a passage which you can do with nothing
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "in an alcove behind some pillars",
        location: L::VAscendantLight,
        index: 517,
        drop: Drop::Health,
        locks: Powerup(A::DreamBreaker),
    },
    Check {
        description: "on a missable ledge in the centre",
        location: L::MainUnderbelly,
        index: 447,
        drop: Drop::SmallKey,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "the note on a high ledge in the big room",
        location: L::MainUnderbelly,
        index: 702,
        drop: Drop::Note,
        locks: All(&[
            Powerup(A::HeliacalPower),
            Powerup(A::Sunsetter),
            Any(&[
                Powerup(A::ClingGem(6)),
                Powerup(A::SunGreaves),
                Powerup(A::SolarWind),
            ]),
        ]),
    },
    Check {
        description: "black hole parkour behind strikebreak wall",
        location: L::MainUnderbelly,
        index: 835,
        drop: Drop::Ability(A::MartialProwess),
        locks: All(&[
            Powerup(A::SoulCutter),
            Powerup(A::AscendantLight),
            Any(&[
                Powerup(A::HeliacalPower),
                Powerup(A::Sunsetter),
                Powerup(A::SolarWind),
            ]),
        ]),
    },
    Check {
        description: "behind the locked door",
        location: L::HpSave,
        index: 836,
        drop: Drop::Ability(A::HeliacalPower),
        locks: All(&[
            Lock::SmallKey,
            Powerup(A::DreamBreaker),
        ]),
    },
    Check {
        description: "the note behind the locked door",
        location: L::HpSave,
        index: 703,
        drop: Drop::Note,
        locks: All(&[
            Lock::SmallKey,
            Powerup(A::DreamBreaker),
        ]),
    },
    Check {
        description: "the note near the empty bailey entrance",
        location: L::BaileyHole,
        index: 704,
        drop: Drop::Note,
        locks: Any(&[
            Powerup(A::SunGreaves),
            All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
            Powerup(A::ClingGem(6)),
            Powerup(A::AscendantLight),
            Powerup(A::SolarWind)
        ]),
    },
    Check {
        description: "on top of the big building",
        location: L::BaileyHole,
        index: 516,
        drop: Drop::Health,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::Sunsetter)]),
    },
    // Tower Ruins
    Check {
        description: "where cling gem normally is",
        location: L::TowerRuinsKeep,
        index: 89,
        drop: Drop::Ability(A::ClingGem(6)),
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            Powerup(A::SunGreaves),
            All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
        ]),
    },
    Check {
        description: "atop the tower",
        location: L::TowerRuinsKeep,
        index: 56,
        drop: Drop::BigKey,
        locks: All(&[
            Powerup(A::ClingGem(2)),
            Any(&[
                Powerup(A::SunGreaves),
                All(&[
                    Powerup(A::HeliacalPower),
                    Powerup(A::Sunsetter),
                ]),
            ]),
        ]),
    },
    // Twilight Theatre
    Check {
        description: "on a beam in the corner",
        location: L::PillarRoom,
        index: 1080,
        drop: Drop::Ability(A::AerialFinesse),
        locks: Any(&[
            All(&[
              Powerup(A::SunGreaves),
              Any(&[Powerup(A::ClingGem(2)), Powerup(A::SolarWind), Powerup(A::Sunsetter)])
            ]),
            All(&[Powerup(A::SolarWind), Powerup(A::ClingGem(2))])
        ]),
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 949,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 950,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 946,
        drop: Drop::Chair,
        locks: Lock::None,
    },
    Check {
        description: "the chair next to the books",
        location: L::OtherTheatrePath,
        index: 951,
        drop: Drop::Chair,
        locks: Lock::None,
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
        locks: Lock::None,
    },
    Check {
        description: "the goatling who thought the theatre was safe",
        location: L::TheatreEntrance,
        index: 939,
        drop: Drop::Goatling(&[
            "I heard that the theatre was still in good condition...",
            "But it seems even this place has been affected."
        ]),
        locks: Lock::None,
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 944,
        drop: Drop::Goatling(&[
            "Ah nuts....I really wanted to see the show today."
        ]),
        locks: Lock::None,
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 945,
        drop: Drop::Goatling(&[
            "Sorry miss, can't let you in.",
            "Theatre's closed until all the haze is gone."
        ]),
        locks: Lock::None,
    },
    Check {
        description: "hiding amid the boxes",
        location: L::MainTheatre,
        index: 843,
        drop: Drop::Health,
        locks: Lock::None,
    },
    Check {
        description: "the goatling that will kill again",
        location: L::MainTheatre,
        index: 936,
        drop: Drop::Goatling(&[
            "please leave me alone?",
            "i will [#cf2525](kill again)"
        ]),
        locks: Any(&[
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower)]),
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(6))
        ]),
    },
    Check {
        description: "the chair near the courtyard",
        location: L::MainTheatre,
        index: 947,
        drop: Drop::Chair,
        locks: Any(&[
            Powerup(A::ClingGem(4)),
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower), Powerup(A::SunGreaves)]),
        ]),
    },
    Check {
        description: "the chair in the soul cutter zone",
        location: L::MainTheatre,
        index: 948,
        drop: Drop::Chair,
        locks: All(&[
            Powerup(A::SoulCutter),
            Powerup(A::SoulCutter),
            Powerup(A::DreamBreaker),
            Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(4))]),
        ]),
    },
    Check {
        description: "behind three maximum security cages",
        location: L::MainTheatre,
        index: 802,
        drop: Drop::BigKey,
        // there's one gap in the open green room with enemies which is too big
        locks: All(&[
            Powerup(A::DreamBreaker),
            Powerup(A::SoulCutter), 
            Powerup(A::SoulCutter), 
            Powerup(A::ClingGem(6)),
            Any(&[
                Powerup(A::HeliacalPower),
                Powerup(A::SolarWind)
            ]),
        ]),
    },
    Check {
        description: "where soul cutter normally is",
        location: L::MainTheatre,
        index: 1079,
        drop: Drop::Ability(A::SoulCutter),
        locks: All(&[
            Powerup(A::DreamBreaker),
            Powerup(A::SoulCutter), 
            Powerup(A::SoulCutter),
            Any(&[
                Powerup(A::ClingGem(6)),
                Powerup(A::SunGreaves),
            ]),
        ]),
    },
    Check {
        description: "in the back on a pillar",
        location: L::MainTheatre,
        index: 844,
        drop: Drop::Health,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(6))]),
    },
    Check {
        description: "behind the locked door",
        location: L::MainTheatre,
        index: 1081,
        drop: Drop::Ability(A::Empathy),
        locks: All(&[
            Lock::SmallKey,
            Powerup(A::DreamBreaker),
            Any(&[
                Powerup(A::SunGreaves),
                Powerup(A::ClingGem(2)),
            ]),
        ]),
    },
];
