use super::*;

use Ability as A;
use Location as L;
use Lock::{Any, All};
pub const CHECKS: [Check; 81] = [
    // dream breaker is randomised with random start
    Check {
        description: "where dream breaker normally is",
        location: L::VDreamBreaker,
        index: 355,
        drop: Drop::Ability(A::DreamBreaker),
        locks: Any(&[]),
    },
    Check {
        description: "where the first health piece is",
        location: L::EarlyPrison,
        index: 214,
        drop: Drop::Health,
        locks: Any(&[
            Any(&[
                A::Sunsetter.into(),
                All(&[A::AscendantLight.into(), A::DreamBreaker.into()]),
                // just enough space to do this
                All(&[A::Slide.into(), A::SolarWind.into()]),
                A::ClingGem(4).into(),
                A::SunGreaves.into(),
                A::HeliacalPower.into(),
            ]),
            // you can drop down from the entrance
            Location::CsMain.into(),
        ]),
    },
    Check {
        description: "where slide normally is",
        location: L::LatePrison,
        index: 356,
        drop: Drop::Ability(A::Slide),
        locks: Any(&[
            A::Slide.into(),
            A::SunGreaves.into(),
            All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
            A::ClingGem(2).into(),
        ]),
    },
    Check {
        description: "black hole parkour off the beaten path",
        location: L::LatePrison,
        index: 357,
        drop: Drop::Ability(A::GoodGraces),
        locks: Any(&[
            A::ClingGem(6).into(),
            All(&[A::AscendantLight.into(), A::DreamBreaker.into(), A::SunGreaves.into()]),
        ]),
    },
    Check {
        description: "up in the rafters",
        location: L::StrongEyes,
        index: 186,
        drop: Drop::SmallKey,
        locks: Any(&[
            All(&[
                A::SunGreaves.into(),
                Any(&[
                    A::Sunsetter.into(),
                    All(&[A::Slide.into(), A::SolarWind.into()])
                ]),
            ]),
            A::ClingGem(4).into(),
        ]),
    },
    Check {
        description: "missable high walled room",
        location: L::StrongEyes,
        index: 215,
        drop: Drop::Health,
        locks: Any(&[A::SunGreaves.into(), A::ClingGem(4).into()]),
    },
    Check {
        description: "strong eyes' lair",
        location: L::StrongEyes,
        index: 185,
        drop: Drop::SmallKey,
        locks: Any(&[A::DreamBreaker.into()]),
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
        locks: Any(&[]),
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 634,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 635,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 636,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "where indignation normally is",
        location: L::CsMain,
        index: 787,
        drop: Drop::Ability(A::Indignation),
        locks: Any(&[]),
    },
    Check {
        description: "chillin' on a ledge by the window",
        location: L::CsMain,
        index: 444,
        drop: Drop::SmallKey,
        locks: Any(&[
            A::Sunsetter.into(),
            A::SunGreaves.into(),
            A::HeliacalPower.into(),
            A::ClingGem(2).into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "where the professional normally is",
        location: L::CsMain,
        index: 791,
        drop: Drop::Ability(A::Professional),
        locks: All(&[
            Lock::SmallKey,
            A::DreamBreaker.into()
        ]),
    },
    Check {
        description: "tucked deep in a corner in the bouncer room",
        location: L::CsMain,
        index: 495,
        drop: Drop::Health,
        locks: Any(&[
            A::ClingGem(6).into(),
            All(&[A::SunGreaves.into(), A::HeliacalPower.into()]),
        ]),
    },
    Check {
        description: "the extremely slappable wheel guy room",
        location: L::CsMain,
        index: 494,
        drop: Drop::Health,
        locks: Any(&[
            All(&[A::AscendantLight.into(), A::DreamBreaker.into()]),
            A::SunGreaves.into(),
            A::ClingGem(2).into(),
        ]),
    },
    Check {
        description: "the old softlock room",
        location: L::CsOldSoftlockRoom,
        index: 498,
        drop: Drop::Health,
        locks: Any(&[
            A::ClingGem(6).into(),
            All(&[A::Slide.into(), A::SolarWind.into(), A::SunGreaves.into(), A::HeliacalPower.into()]),
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
            A::ClingGem(6).into(),
            All(&[A::Slide.into(), A::SolarWind.into(), A::SunGreaves.into(), A::HeliacalPower.into()]),
        ]),
    },
    Check {
        description: "cool moon room",
        location: L::CsTheatreEntrance,
        index: 790,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            A::ClingGem(6).into(),
            All(&[A::Slide.into(), A::SolarWind.into(), A::SunGreaves.into(), A::HeliacalPower.into()]),
        ]),
    },
    Check {
        description: "through the wallkick tunnel",
        location: L::CsMain,
        index: 443,
        drop: Drop::SmallKey,
        locks: Any(&[A::SunGreaves.into(), A::ClingGem(6).into()]),
    },
    Check {
        description: "in the pit next to the dungeon entrance",
        location: L::CsMain,
        index: 497,
        drop: Drop::Health,
        locks: Any(&[
            A::SunGreaves.into(),
            A::HeliacalPower.into(),
            A::ClingGem(6).into(),
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
            A::SunGreaves.into(),
            A::HeliacalPower.into(),
            A::ClingGem(4).into(),
            All(&[A::Slide.into(), A::SunGreaves.into()])
        ]),
    },
    Check {
        description: "on the ledge above the bailey entrance",
        location: L::CsMain,
        index: 496,
        drop: Drop::Health,
        locks: Any(&[
            All(&[A::SunGreaves.into(), A::HeliacalPower.into()]),
            A::ClingGem(4).into(),
            All(&[A::Slide.into(), A::SolarWind.into(), A::HeliacalPower.into()]),
            All(&[A::Slide.into(), A::SolarWind.into(), A::SunGreaves.into()]),
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
        locks: Any(&[]),
    },
    Check {
        description: "the chair in the gazebo",
        location: L::CsMain,
        index: 637,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "next to a bouncer in the massive room",
        location: L::CsMain,
        index: 788,
        drop: Drop::Ability(A::Pilgrimage),
        locks: Any(&[
            All(&[
                A::AscendantLight.into(),
                A::DreamBreaker.into(),
                Any(&[
                    A::Sunsetter.into(),
                    A::SunGreaves.into(),
                    All(&[A::Slide.into(),A::SolarWind.into()]),
                ]),
            ]),
            All(&[A::SunGreaves.into(), A::HeliacalPower.into()]),
            All(&[
                A::ClingGem(6).into(), 
                Any(&[
                    A::HeliacalPower.into(),
                    A::Sunsetter.into(),
                    All(&[A::Slide.into(),A::SolarWind.into()]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "in the room with two other ones to each side",
        location: L::CsMain,
        index: 789,
        drop: Drop::Ability(A::GoodGraces),
        locks: Any(&[A::ClingGem(6).into(), A::SunGreaves.into()]),
    },
    // Listless Library
    Check {
        description: "the chair at the entrance",
        location: L::MainLibrary,
        index: 241,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "where sun greaves normally are",
        location: L::LibSaveNearGreaves,
        index: 267,
        drop: Drop::Ability(A::SunGreaves),
        locks: Any(&[
            A::DreamBreaker.into()
        ]),
    },
    Check {
        description: "the chair after the normal sun greaves location",
        location: L::LibSaveNearGreaves,
        index: 243,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "the note next to the egg nest",
        location: L::MainLibrary,
        index: 240,
        drop: Drop::Note,
        locks: Any(&[
            A::SunGreaves.into(),
            A::ClingGem(4).into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "the chair next to the egg nest",
        location: L::MainLibrary,
        index: 242,
        drop: Drop::Chair,
        locks: Any(&[
            A::SunGreaves.into(),
            A::ClingGem(4).into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "in the buttress room",
        location: L::MainLibrary,
        index: 213,
        drop: Drop::Health,
        locks: Any(&[A::SunGreaves.into(), A::ClingGem(4).into()]),
    },
    Check {
        description: "in the hay behind the locked door",
        location: L::Restricted,
        index: 268,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            All(&[A::Slide.into(), A::SolarWind.into()]),
            A::SunGreaves.into(),
            A::HeliacalPower.into(),
            A::ClingGem(4).into(),
        ]),
    },
    Check {
        description: "tucked deep behind the locked door",
        location: L::Restricted,
        index: 214,
        drop: Drop::Health,
        locks: Any(&[A::SunGreaves.into(), A::ClingGem(4).into()]),
    },
    // Sansa Keep
    Check {
        description: "the goatling sad about the lack of furniture",
        location: L::SansaKeep,
        index: 323,
        drop: Drop::Goatling(&[
            "They took away all my furniture."
        ]),
        locks: Any(&[]),
    },
    Check {
        description: "the goatling collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 324,
        drop: Drop::Goatling(&[
            "[6rr](.....c.....y..u...i....y.......ce....)"
        ]),
        locks: Any(&[]),
    },
    Check {
        description: "the chair collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 325,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "where strikebreak normally is",
        location: L::SansaKeep,
        index: 393,
        drop: Drop::Ability(A::Strikebreak),
        locks: Any(&[
            All(&[
                A::Strikebreak.into(),
                A::DreamBreaker.into(),
                Any(&[
                    A::SunGreaves.into(),
                    A::HeliacalPower.into(),
                    A::ClingGem(4).into(),
                    All(&[A::Slide.into(), A::SolarWind.into()]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "where sunsetter normally is",
        location: L::Sunsetter,
        index: 392,
        drop: Drop::Ability(A::Sunsetter),
        locks: All(&[
            A::DreamBreaker.into(),
            Any(&[
                A::Sunsetter.into(),
                A::HeliacalPower.into(),
                A::SunGreaves.into(),
                A::ClingGem(2).into()
            ]),
        ]),
    },
    Check {
        description: "in an alcove next to the locked door",
        location: L::Sunsetter,
        index: 251,
        drop: Drop::Health,
        locks: Any(&[
            A::Sunsetter.into(),
            A::SunGreaves.into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "in the room with a lever on each side",
        location: L::SansaKeep,
        index: 226,
        drop: Drop::SmallKey,
        locks: Any(&[A::DreamBreaker.into()]),
    },
    Check {
        description: "tucked near the theatre entrance",
        location: L::SansaKeep,
        index: 394,
        drop: Drop::Ability(A::ClearMind),
        locks: Any(&[
            A::Sunsetter.into(),
            A::HeliacalPower.into(),
            A::SunGreaves.into(),
            A::ClingGem(4).into(),
        ]),
    },
    Check {
        description: "at the end of the parkour",
        location: L::SansaKeep,
        index: 227,
        drop: Drop::BigKey,
        locks: Any(&[
            All(&[
                A::AscendantLight.into(),
                A::DreamBreaker.into(),
                Any(&[
                    All(&[
                        A::ClingGem(4).into(),
                        Any(&[A::Sunsetter.into(), A::SunGreaves.into()]),
                    ]),
                    All(&[A::Sunsetter.into(), A::SunGreaves.into()]),
                ]),
            ]),
            All(&[
                A::DreamBreaker.into(),
                A::Slide.into(),
                A::SolarWind.into(),
                A::Sunsetter.into(),
                A::ClingGem(2).into(),
                A::SunGreaves.into(),
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
                A::AscendantLight.into(),
                A::DreamBreaker.into(),
                Any(&[
                    All(&[
                        A::ClingGem(4).into(),
                        Any(&[A::Sunsetter.into(), A::SunGreaves.into()]),
                    ]),
                    All(&[A::Sunsetter.into(), A::SunGreaves.into()]),
                ]),
            ]),
            All(&[
                A::DreamBreaker.into(),
                A::Slide.into(),
                A::SolarWind.into(),
                A::Sunsetter.into(),
                A::ClingGem(2).into(),
                A::SunGreaves.into(),
            ]),
        ]),
    },
    // Empty Bailey
    Check {
        description: "the goatling who's hiding",
        location: L::EmptyBailey,
        index: 88,
        drop: Drop::Goatling(&["...i'm not here."]),
        locks: Any(&[A::Slide.into()]),
    },
    Check {
        description: "in the building you slide into",
        location: L::EmptyBailey,
        index: 56,
        drop: Drop::SmallKey,
        locks: Any(&[A::Slide.into()]),
    },
    Check {
        description: "guarded by the hand and soldier",
        location: L::EmptyBailey,
        index: 55,
        drop: Drop::BigKey,
        locks: Any(&[
            A::Sunsetter.into(),
            A::SunGreaves.into(),
            A::ClingGem(4).into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "where solar wind normally is",
        location: L::EmptyBailey,
        index: 104,
        drop: Drop::Ability(A::SolarWind),
        locks: Any(&[
            All(&[
                A::Slide.into(),
                A::DreamBreaker.into(),
                Any(&[
                    A::SolarWind.into(),
                    A::HeliacalPower.into(),
                    A::SunGreaves.into(),
                ]),
            ]),
        ]),
    },
    Check {
        description: "in the tower in the middle",
        location: L::EmptyBailey,
        index: 66,
        drop: Drop::Health,
        locks: Any(&[
            All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
            A::SunGreaves.into(),
            // you can jump down from cheese bell
            All(&[A::Slide.into(), A::SolarWind.into(), A::ClingGem(6).into()]),
        ]),
    },
    Check {
        description: "under the cheese bell",
        location: L::EmptyBailey,
        index: 105,
        drop: Drop::Ability(A::Empathy),
        locks: Any(&[
            All(&[
                A::Slide.into(),
                A::SolarWind.into(),
                Any(&[
                    A::ClingGem(6).into(),
                    All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
                ]),
            ]),
            
            All(&[A::Sunsetter.into(), A::SunGreaves.into()]),

        ]),
    },
    // Underbelly
    Check {
        description: "near the entrance from sansa keep",
        location: L::SansaHole,
        index: 515,
        drop: Drop::Health,
        locks: Any(&[
            A::SunGreaves.into(),
            All(&[A::Sunsetter.into(), A::HeliacalPower.into()]),
        ]),
    },
    Check {
        description: "the soul cutter lever room",
        location: L::SansaHole,
        index: 446,
        drop: Drop::BigKey,
        locks: Any(&[
            All(&[
                A::DreamBreaker.into(),
                A::Sunsetter.into(),
                Any(&[
                    All(&[
                        A::SoulCutter.into(),
                        Any(&[A::AscendantLight.into(), A::ClingGem(6).into()]),
                    ]),
                    All(&[A::SunGreaves.into(), A::Slide.into(), A::SolarWind.into()]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "where ascendant light normally is",
        location: L::VAscendantLight,
        index: 834,
        drop: Drop::Ability(A::AscendantLight),
        // you can go through the dark area and there's a passage which you can do with nothing
        locks: Any(&[A::DreamBreaker.into()]),
    },
    Check {
        description: "in an alcove behind some pillars",
        location: L::VAscendantLight,
        index: 517,
        drop: Drop::Health,
        locks: Any(&[A::DreamBreaker.into()]),
    },
    Check {
        description: "on a missable ledge in the centre",
        location: L::MainUnderbelly,
        index: 447,
        drop: Drop::SmallKey,
        locks: Any(&[
            A::Sunsetter.into(),
            All(&[A::Slide.into(), A::SolarWind.into()]),
        ]),
    },
    Check {
        description: "the note on a high ledge in the big room",
        location: L::MainUnderbelly,
        index: 702,
        drop: Drop::Note,
        locks: Any(&[
            All(&[
                A::HeliacalPower.into(),
                A::Sunsetter.into(),
                Any(&[
                    A::ClingGem(6).into(),
                    A::SunGreaves.into(),
                    All(&[A::Slide.into(), A::SolarWind.into()]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "black hole parkour behind strikebreak wall",
        location: L::MainUnderbelly,
        index: 835,
        drop: Drop::Ability(A::MartialProwess),
        locks: Any(&[
            All(&[
                A::Strikebreak.into(),
                A::AscendantLight.into(),
                A::DreamBreaker.into(),
                Any(&[
                    A::HeliacalPower.into(),
                    A::SunGreaves.into(),
                    A::Sunsetter.into(),
                    All(&[A::Slide.into(), A::SolarWind.into()]),
                ]),
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
            A::DreamBreaker.into(),
        ]),
    },
    Check {
        description: "the note behind the locked door",
        location: L::HpSave,
        index: 703,
        drop: Drop::Note,
        locks: All(&[
            Lock::SmallKey,
            A::DreamBreaker.into(),
        ]),
    },
    Check {
        description: "the note near the empty bailey entrance",
        location: L::BaileyHole,
        index: 704,
        drop: Drop::Note,
        locks: Any(&[
            A::SunGreaves.into(),
            All(&[A::HeliacalPower.into(), A::Sunsetter.into()]),
            A::ClingGem(6).into(),
            All(&[A::AscendantLight.into(), A::DreamBreaker.into()]),
            All(&[A::Slide.into(), A::SolarWind.into()])
        ]),
    },
    Check {
        description: "on top of the big building",
        location: L::BaileyHole,
        index: 516,
        drop: Drop::Health,
        locks: Any(&[A::SunGreaves.into(), A::Sunsetter.into()]),
    },
    // Tower Ruins
    Check {
        description: "where cling gem normally is",
        location: L::TowerRuinsKeep,
        index: 89,
        drop: Drop::Ability(A::ClingGem(6)),
        locks: Any(&[
            A::ClingGem(6).into(),
            A::SunGreaves.into(),
            All(&[A::HeliacalPower.into(), A::Sunsetter.into()]),
        ]),
    },
    Check {
        description: "atop the tower",
        location: L::TowerRuinsKeep,
        index: 56,
        drop: Drop::BigKey,
        locks: Any(&[
            All(&[
                A::ClingGem(2).into(),
                Any(&[
                    A::SunGreaves.into(),
                    All(&[
                        A::HeliacalPower.into(),
                        A::Sunsetter.into(),
                    ]),
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
                Any(&[A::SunGreaves.into(), A::ClingGem(2).into()]),
                Any(&[A::Slide.into(), A::SolarWind.into()]),
            ]),
            All(&[A::ClingGem(2).into(), A::SunGreaves.into()]),
            All(&[A::Sunsetter.into(), A::SunGreaves.into()]),
        ]),
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 949,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 950,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 946,
        drop: Drop::Chair,
        locks: Any(&[]),
    },
    Check {
        description: "the chair next to the books",
        location: L::OtherTheatrePath,
        index: 951,
        drop: Drop::Chair,
        locks: Any(&[]),
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
        locks: Any(&[]),
    },
    Check {
        description: "the goatling who thought the theatre was safe",
        location: L::TheatreEntrance,
        index: 939,
        drop: Drop::Goatling(&[
            "I heard that the theatre was still in good condition...",
            "But it seems even this place has been affected."
        ]),
        locks: Any(&[]),
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 944,
        drop: Drop::Goatling(&[
            "Ah nuts....I really wanted to see the show today."
        ]),
        locks: Any(&[]),
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 945,
        drop: Drop::Goatling(&[
            "Sorry miss, can't let you in.",
            "Theatre's closed until all the haze is gone."
        ]),
        locks: Any(&[]),
    },
    Check {
        description: "hiding amid the boxes",
        location: L::MainTheatre,
        index: 843,
        drop: Drop::Health,
        locks: Any(&[]),
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
            All(&[A::Slide.into(), A::SolarWind.into(), A::HeliacalPower.into()]),
            A::SunGreaves.into(),
            A::ClingGem(6).into()
        ]),
    },
    Check {
        description: "the chair near the courtyard",
        location: L::MainTheatre,
        index: 947,
        drop: Drop::Chair,
        locks: Any(&[
                A::ClingGem(4).into(),
                All(&[A::Slide.into(), A::SolarWind.into(), A::HeliacalPower.into(), A::SunGreaves.into()]),
            ]),
    },
    Check {
        description: "the chair in the soul cutter zone",
        location: L::MainTheatre,
        index: 948,
        drop: Drop::Chair,
        locks: Any(&[
            All(&[
                A::Strikebreak.into(),
                A::SoulCutter.into(),
                A::DreamBreaker.into(),
                Any(&[A::SunGreaves.into(), A::ClingGem(4).into()]),
            ]),
        ]),
    },
    Check {
        description: "behind three maximum security cages",
        location: L::MainTheatre,
        index: 802,
        drop: Drop::BigKey,
        // there's one gap in the open green room with enemies which is too big
        locks: All(&[
            A::DreamBreaker.into(),
            A::Strikebreak.into(), 
            A::SoulCutter.into(), 
            A::ClingGem(6).into(),
            Any(&[
                A::HeliacalPower.into(),
                A::SunGreaves.into(),
                All(&[A::Slide.into(), A::SolarWind.into()])
            ]),
        ]),
    },
    Check {
        description: "where soul cutter normally is",
        location: L::MainTheatre,
        index: 1079,
        drop: Drop::Ability(A::SoulCutter),
        locks: All(&[
            A::DreamBreaker.into(),
            A::Strikebreak.into(), 
            A::SoulCutter.into(),
            Any(&[
                A::ClingGem(6).into(),
                A::SunGreaves.into(),
            ]),
        ]),
    },
    Check {
        description: "in the back on a pillar",
        location: L::MainTheatre,
        index: 844,
        drop: Drop::Health,
        locks: Any(&[A::SunGreaves.into(), A::ClingGem(6).into()]),
    },
    Check {
        description: "behind the locked door",
        location: L::MainTheatre,
        index: 1081,
        drop: Drop::Ability(A::Empathy),
        locks: All(&[
            Lock::SmallKey,
            A::DreamBreaker.into(),
            Any(&[
                A::SunGreaves.into(),
                A::ClingGem(2).into(),
            ]),
        ]),
    },
];
