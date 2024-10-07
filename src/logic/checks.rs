use super::*;

use Ability as A;
use Location as L;
use Lock::{All, Any, Movement as Powerup, Location as Loc, Trick};
// for some reason doesn't import properly as just Trick
use super::Trick as T;
use Difficulty as D;

pub const CHECKS: [Check; 92] = [
    Check {
        description: "the goatling who fell out his cage",
        location: L::EarlyPrison,
        index: 433,
        drop: Drop::Goatling(&[
            "Oh[3rr](...)sorry, I didn't mean to[3rr](...)fall out of my cage[3rr](...)",
            "I hope[3rr](...)the [#cf2525](princess) won't be upset with me[3rr](...)"
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the time trial in the starting room",
        location: L::EarlyPrison,
        index: 503,
        drop: Drop::Ability(A::SolSister),
        key_index: None,
        trial: Some(486),
        // not final logic
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter)]),
    },
    Check {
        description: "where dream breaker normally is",
        location: L::EarlyPrison,
        index: 501,
        drop: Drop::Ability(A::DreamBreaker),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::DreamBreaker),
            All(&[
                Powerup(A::SunGreaves),
                Trick(T::Movement, D::Expert),
                Trick(T::OneWall, D::Advanced),
            ]),
            All(&[
                Powerup(A::SunGreaves),
                Powerup(A::SolarWind),
                Trick(T::Movement, D::Advanced),
                Trick(T::OneWall, D::Advanced),
            ]),
        ]),
    },
    Check {
        description: "where the first health piece is",
        location: L::EarlyPrison,
        index: 283,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Any(&[
                Powerup(A::Sunsetter),
                Powerup(A::AscendantLight),
                // just enough space to do this
                All(&[Powerup(A::SolarWind), Trick(T::Movement, D::Normal)]),
                All(&[Powerup(A::ClingGem(4)), Trick(T::ClingAbuse, D::Normal)]),
                Powerup(A::HeliacalPower),
            ]),
            // you can drop down from the entrance
            Loc(Location::CsMain),
        ]),
    },
    Check {
        description: "where slide normally is",
        location: L::LatePrison,
        index: 502,
        drop: Drop::Ability(A::Slide),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::Slide),
            Powerup(A::SunGreaves),
            All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
            Powerup(A::ClingGem(2)),
            All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::Sunsetter), Trick(T::Movement, D::Advanced)]),
        ]),
    },
    Check {
        description: "black hole parkour off the beaten path",
        location: L::LatePrison,
        index: 504,
        drop: Drop::Ability(A::GoodGraces),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::AscendantLight), Powerup(A::SunGreaves)]),
            All(&[Powerup(A::SunGreaves), Powerup(A::SolarWind), Trick(T::Movement, D::Advanced), Trick(T::OneWall, D::Advanced)]),
        ]),
    },
    Check {
        description: "up in the rafters",
        location: L::StrongEyes,
        index: 255,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[
                Powerup(A::SunGreaves),
                Any(&[
                    Powerup(A::Sunsetter),
                    Powerup(A::SolarWind)
                ]),
            ]),
            All(&[Powerup(A::ClingGem(4)), Trick(T::ClingAbuse, D::Advanced)]),
        ]),
    },
    Check {
        description: "missable high walled room",
        location: L::StrongEyes,
        index: 284,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: All(&[
            Any(&[
                Powerup(A::DreamBreaker),
                All(&[Powerup(A::Sunsetter), Trick(T::Knowledge, D::Normal)]),
            ]),
            Any(&[
                All(&[Powerup(A::SunGreaves)]), 
                Powerup(A::ClingGem(4))
            ]),
        ]),
    },
    Check {
        description: "strong eyes' lair",
        location: L::StrongEyes,
        index: 254,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Powerup(A::DreamBreaker), // not changing, fighting without DB sucks
    },
    Check {
        description: "the goatling lamenting the skill issue of players who need a map",
        location: L::CsMain,
        index: 820,
        drop: Drop::Goatling(&["So many of have been trapped in the dungeon, and for what?"]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "where memento normally is",
        location: L::CsMain,
        index: 998,
        drop: Drop::Ability(A::Memento),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 817,
        drop: Drop::Goatling(&[
            "These [#7c79e8](crystals) are pretty nice, right?",
            "They make me feel safe...",
            "I think i'm gonna lick it. I bet it's full of [#8ada1c, buoy, italics](minerals).",
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 822,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 823,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "a chair next to the goatling who wants to lick the checkpoint",
        location: L::CsMain,
        index: 824,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "where indignation normally is",
        location: L::CsMain,
        index: 994,
        drop: Drop::Ability(A::Indignation),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "chillin' on a ledge by the window",
        location: L::CsMain,
        index: 540,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::Sunsetter),
            All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::ClingGem(2)), Trick(T::ClingAbuse, D::Normal)]),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "the goatling who wanted to see the armour display",
        location: L::CsMain,
        index: 821,
        drop: Drop::Goatling(&[
            "There was supposed to be a [#ffdb6b](new armor display) in the [#cf2525](library) I wanted to see.",
            "But with the state of the castle,  I can't find the entrance anywhere!",
            "I miss that [#ba7f27](comfy hay)..."
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the time trial behind a locked door",
        location: L::CsMain,
        index: 999,
        drop: Drop::Ability(A::Professional),
        key_index: None,
        trial: Some(969),
        locks: Lock::SmallKey, // You can easily do this with nothing but ability to hit the crystal.
    },
    Check {
        description: "tucked deep in a corner in the bouncer room",
        location: L::CsMain,
        index: 592,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
        ]),
    },
    Check {
        description: "the extremely slappable wheel guy room",
        location: L::CsMain,
        index: 591,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::AscendantLight),
            All(&[Powerup(A::SunGreaves), Trick(T::OneWall, D::Normal)]),
            Powerup(A::ClingGem(2)),
        ]),
    },
    Check {
        description: "the old softlock room",
        location: L::CsOldSoftlockRoom,
        index: 595,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[Powerup(A::ClingGem(6)), Trick(T::ClingAbuse, D::Expert), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Powerup(A::AscendantLight)]), // Intended way
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::SunGreaves), Powerup(A::Sunsetter), Trick(T::OneWall, D::Advanced), Trick(T::SunsetterAbuse, D::Advanced)])
        ]),
    },
    Check {
        description: "the goatling about to jump into the haze",
        location: L::CsOldSoftlockRoom,
        index: 819,
        drop: Drop::Goatling(&[
            "Oh, thanks for breaking that wall down.",
            "Thought I was gonna have to jump into the haze..."
        ]),
        key_index: None,
        trial: None,
        locks: Any(&[ // Just need to break the wall.. nothing new
            Powerup(A::DreamBreaker),
            All(&[
                Powerup(A::Sunsetter),
                Trick(T::Knowledge, D::Normal),
            ])
        ]),
    },
    Check {
        description: "cool moon room",
        location: L::CsTheatreEntrance,
        index: 997,
        drop: Drop::Ability(A::ClearMind),
        key_index: None,
        trial: None,
        locks: Any(&[ // Cross gap then climb the room
            All(&[Powerup(A::ClingGem(6)), Trick(T::ClingAbuse, D::Normal)]),
            All(&[Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::Sunsetter), Powerup(A::ClingGem(4)), Trick(T::ClingAbuse, D::Normal)]),
            All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves), Powerup(A::ClingGem(2))]),
            All(&[Powerup(A::SolarWind), Powerup(A::ClingGem(2)), Powerup(A::SunGreaves)]),
        ]),
    },
    Check {
        description: "through the wallkick tunnel",
        location: L::CsMain,
        index: 539,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: All(&[
            Any(&[// Activate the switch OR skip it
                Powerup(A::DreamBreaker),
                All(&[Powerup(A::Sunsetter), Trick(T::Knowledge, D::Normal)]),
                All(&[
                    Powerup(A::SunGreaves),
                    Powerup(A::HeliacalPower),
                    Trick(T::OneWall, D::Normal),
                ]),
                All(&[
                    Powerup(A::SolarWind),
                    Powerup(A::HeliacalPower),
                    Trick(T::Movement, D::Normal)
                ]),
                All(&[
                    Powerup(A::SolarWind),
                    Trick(T::Movement, D::Advanced)
                ]),
            ]),
            Any(&[ // Getting here is not an issue really.
                Powerup(A::SunGreaves), Powerup(A::ClingGem(6)),
                All(&[
                    Powerup(A::SunGreaves),
                    Powerup(A::SolarWind),
                    Trick(T::Movement, D::Normal),
                    Trick(T::OneWall, D::Normal),
                ]),
                All(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SolarWind)
                ]),
                All(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SunGreaves)
                ]),
                All(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::HeliacalPower),
                    Trick(T::Movement, D::Advanced),
                    Trick(T::OneWall, D::Normal),

                ])
            ]),
        ]),
    },
    Check {
        description: "in the pit next to the dungeon entrance",
        location: L::CsTheatreEntryNearPrison, // Change Node so that we can infer the logic for getting to said node
        index: 594,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::ClingGem(4)),
            Powerup(A::HeliacalPower),
        ]),
    },
    Check {
        description: "the goatling that calls you bubble girl",
        location: L::CsTheatreEntryNearPrison, // Same as above, lower logic requirements that way
        index: 816,
        drop: Drop::Goatling(&[
            "I was supposed to go help in the [#cf2525](theatre), but I can't really get through here.",
            "I just don't really wanna touch the [#cf2525](bubbles)...",
            "What? I dont have a problem. You go touch 'em then, bubble girl."
        ]),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "on the ledge above the bailey entrance",
        location: L::CsMain,
        index: 593,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[ // Not much to add here really
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Trick(T::Movement, D::Normal)]),
            Powerup(A::ClingGem(4)),
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the goatling in the gazebo",
        location: L::CsMain,
        index: 818,
        drop: Drop::Goatling(&[
            "The princess used to love having afternoon tea here.",
            "But the handmaiden has run out of her special ingredient.",
            "I guess the princess doesn't really want anybody else's tea...",
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the chair in the gazebo",
        location: L::CsMain,
        index: 825,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "next to a bouncer in the massive room",
        location: L::CsMain,
        index: 995,
        drop: Drop::Ability(A::Pilgrimage),
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[
                Powerup(A::AscendantLight),
                Trick(T::Movement, D::Normal),
                Any(&[
                    Powerup(A::SunGreaves),
                    Powerup(A::SolarWind),
                ]),
            ]),
            All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower), Trick(T::OneWall, D::Advanced)]),
            All(&[
                Powerup(A::ClingGem(6)), 
                Any(&[
                    Powerup(A::HeliacalPower),
                    Powerup(A::Sunsetter),
                    Powerup(A::SolarWind),
                    Powerup(A::AscendantLight),
                ]),
            ]),
        ]),
    },
    Check {
        description: "in the room with two other ones to each side",
        location: L::CsMain,
        index: 996,
        drop: Drop::Ability(A::GoodGraces),
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[
                Powerup(A::ClingGem(6)),
                Any(&[
                    Powerup(A::Sunsetter),
                    Powerup(A::HeliacalPower),
                    Trick(T::ClingAbuse, D::Normal),
                ]),
            ]),
            Powerup(A::SunGreaves),
            All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            All(&[Powerup(A::SolarWind), Trick(T::Movement, D::Normal)]),
        ]),
    },
    // Listless Library
    Check {
        description: "the chair at the entrance",
        location: L::MainLibrary,
        index: 289,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the time trial amidst the books",
        location: L::MainLibrary,
        index: 325,
        drop: Drop::Ability(A::Sleepytime),
        key_index: None,
        // not final logic
        trial: Some(314),
        locks: All(&[
            Any(&[
                Powerup(A::DreamBreaker),
                All(&[
                    Powerup(A::Sunsetter),
                    Trick(T::Knowledge, D::Normal)
                ])
            ]),
            Any(&[
                All(&[Powerup(A::Sunsetter), Trick(T::Movement, D::Advanced)]),
                All(&[Powerup(A::SunGreaves), Trick(T::Movement, D::Normal)]),
                All(&[Powerup(A::SolarWind), Trick(T::Movement,D::Normal)]),
                All(&[Powerup(A::ClingGem(6)), Trick(T::Movement, D::Advanced), Trick(T::ClingAbuse, D::Normal)]),
                All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower), Powerup(A::SolarWind)]),
            ]),
        ]),
    },
    Check {
        description: "where sun greaves normally is",
        location: L::LibSaveNearGreaves,
        index: 324,
        drop: Drop::Ability(A::SunGreaves),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::DreamBreaker),
            All(&[Trick(T::Knowledge, D::Advanced), Powerup(A::Sunsetter)]),
        ]),
    },
    Check {
        description: "the chair after the normal sun greaves location",
        location: L::LibSaveNearGreaves,
        index: 291,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the note next to the egg nest",
        location: L::MainLibrary,
        index: 288,
        drop: Drop::Note,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "the chair next to the egg nest",
        location: L::MainLibrary,
        index: 290,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "in the buttress room",
        location: L::MainLibrary,
        index: 237,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(4))]),
    },
    Check {
        description: "in the hay behind the locked door",
        location: L::Restricted,
        index: 326,
        drop: Drop::Ability(A::ClearMind),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SolarWind),
            All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Expert)]),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "tucked deep behind the locked door",
        location: L::Restricted,
        index: 238,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SolarWind),
            All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Expert)]),
            Powerup(A::ClingGem(4)),
        ]),
    },
    // Sansa Keep
    Check {
        description: "the goatling sad about the lack of furniture",
        location: L::SansaKeep,
        index: 475,
        drop: Drop::Goatling(&[
            "They took away all my furniture."
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 476,
        drop: Drop::Goatling(&[
            "[6rr](.....c.....y..u...i....y.......ce....)"
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the chair collapsing out of reality",
        location: L::SkCastleRampEntry,
        index: 477,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "where strikebreak normally is",
        location: L::SansaKeep,
        index: 565,
        drop: Drop::Ability(A::Strikebreak),
        key_index: None,
        trial: None,
        locks:  All(&[
            Powerup(A::Strikebreak),
            Any(&[
                All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]), // Only at advanced bc of the forward backflip that is harder on controller.
                Powerup(A::ClingGem(4)),
                Powerup(A::SolarWind),
            ]),
        ]),
    },
    Check {
        description: "where sunsetter normally is",
        location: L::Sunsetter,
        index: 564,
        drop: Drop::Ability(A::Sunsetter),
        key_index: None,
        trial: None,
        locks: All(&[
            Powerup(A::DreamBreaker),
            Any(&[
                Powerup(A::Sunsetter),
                All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]), // Only at advanced bc of the forward backflip that is harder on controller.
                All(&[Powerup(A::ClingGem(2)), Trick(T::ClingAbuse, D::Normal)]),
            ]),
        ]),
    },
    Check {
        description: "in an alcove next to the locked door",
        location: L::Sunsetter,
        index: 330,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::SunGreaves),
            Powerup(A::SolarWind),
            Trick(T::Movement, D::Normal), // Can legit get this sphere 0 with nothing.
        ]),
    },
    Check {
        description: "in the room with a lever on each side",
        location: L::SansaKeep,
        index: 304,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[Powerup(A::Sunsetter), Trick(T::Knowledge, D::Normal)]),
            Powerup(A::DreamBreaker),
        ]),
    },
    Check {
        description: "tucked near the theatre entrance",
        location: L::SansaKeep,
        index: 566,
        drop: Drop::Ability(A::ClearMind),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::HeliacalPower),
            Powerup(A::ClingGem(4)),
        ]),
    },
    Check {
        description: "at the end of the parkour",
        location: L::SansaKeep,
        index: 305,
        drop: Drop::BigKey,
        key_index: Some(4),
        trial: None,
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
        description: "the time trial at the end of the parkour",
        location: L::SansaKeep,
        index: 567,
        drop: Drop::Ability(A::Guardian),
        key_index: None,
        // only logic to get here not final
        trial: Some(541),
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter), Powerup(A::AscendantLight)]),
    },
    Check {
        description: "the chair in the middle of the parkour",
        location: L::SansaKeep,
        index: 478,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
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
        index: 115,
        drop: Drop::Goatling(&["...i'm not here."]),
        key_index: None,
        trial: None,
        locks: Powerup(A::Slide), // Theres another way to get TO the item, not putting it in since its a one way unless it is slide though...
    },
    Check {
        description: "in the building you slide into",
        location: L::EmptyBailey,
        index: 70,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Powerup(A::Slide),
    },
    Check {
        description: "guarded by the hand and soldier",
        location: L::EmptyBailey,
        index: 69,
        drop: Drop::BigKey,
        key_index: Some(1),
        trial: None,
        locks: Any(&[
            Powerup(A::Sunsetter),
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(4)),
            Powerup(A::SolarWind),
            All(&[
                Loc(L::EbEntryUnderBelly),
                Trick(T::Movement, D::Advanced), // Can just jump to it from above.
            ]),
        ]),
    },
    Check {
        description: "where solar wind normally is",
        location: L::EmptyBailey,
        index: 148,
        drop: Drop::Ability(A::SolarWind),
        key_index: None,
        trial: None,
        locks: All(&[
            Powerup(A::Slide),
            Any(&[
                Powerup(A::DreamBreaker),
                All(&[
                    Powerup(A::Sunsetter),
                    Trick(T::Knowledge, D::Normal),
                ])
            ]),
            Any(&[
                Powerup(A::SolarWind),
                All(&[Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
            ]),
        ]),
    },
    Check {
        description: "in the tower in the middle",
        location: L::EmptyBailey,
        index: 80,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::Sunsetter),
            All(&[Powerup(A::HeliacalPower), Trick(T::ReverseKick, D::Advanced)]),
            All(&[Powerup(A::SunGreaves), Trick(T::OneWall, D::Normal)]),
            // you can jump down from cheese bell
            All(&[
                Powerup(A::SolarWind), 
                Any(&[
                    Powerup(A::ClingGem(6)),
                    All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
                    All(&[Trick(T::Movement, D::Advanced),Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                    All(&[Powerup(A::ClingGem(4)), Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "under the cheese bell",
        location: L::EmptyBailey,
        index: 149,
        drop: Drop::Ability(A::Empathy),
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[
                Powerup(A::SolarWind),
                Any(&[
                    Powerup(A::ClingGem(6)),
                    All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower), Trick(T::Movement, D::Advanced)]),
                ]),
            ]),
            All(&[Trick(T::Movement, D::Advanced),Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
            All(&[Powerup(A::ClingGem(4)), Powerup(A::Sunsetter), Powerup(A::SunGreaves)]),
        ]),
    },
    Check {
        description: "the locked up time trial",
        location: L::EmptyBailey,
        index: 150,
        drop: Drop::Ability(A::Soldier),
        key_index: None,
        trial: Some(128),
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter)]),
    },
    // Underbelly
    Check {
        description: "near the entrance from sansa keep",
        location: L::SansaHole,
        index: 614,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SunGreaves),
            All(&[Powerup(A::Sunsetter), Powerup(A::HeliacalPower)]),
        ]),
    },
    Check {
        description: "the soul cutter lever room",
        location: L::SansaHole,
        index: 545,
        drop: Drop::BigKey,
        key_index: Some(2),
        trial: None,
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
        index: 1044,
        drop: Drop::Ability(A::AscendantLight),
        key_index: None,
        // you can go through the dark area and there's a passage which you can do with nothing
        trial: None,
        locks: Any(&[
            Powerup(A::DreamBreaker),
            All(&[
                Powerup(A::Sunsetter),
                Trick(T::Knowledge, D::Expert), // this is at expert bc of fighting the statue...
            ])
        ]),
    },
    Check {
        description: "in an alcove behind some pillars",
        location: L::VAscendantLight,
        index: 616,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[Powerup(A::Sunsetter), Trick(T::Knowledge, D::Normal)]), // normal route into AL 
            Powerup(A::DreamBreaker)
        ]),
    },
    Check {
        description: "on a missable ledge in the centre",
        location: L::MainUnderbelly,
        index: 546,
        drop: Drop::SmallKey,
        key_index: None,
        trial: None,
        locks: Any(&[ // Leaving as is for now.
            Powerup(A::Sunsetter),
            Powerup(A::SolarWind),
        ]),
    },
    Check {
        description: "the note on a high ledge in the big room",
        location: L::MainUnderbelly,
        index: 894,
        drop: Drop::Note,
        key_index: None,
        trial: None,
        locks: All(&[ // Leaving as is for now.
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
        index: 1046,
        drop: Drop::Ability(A::MartialProwess),
        key_index: None,
        trial: None,
        locks: All(&[// Leaving this one out of tricks for now since theres a ton of ways to do it
            Powerup(A::Strikebreak), // Dont know why this one was SoulCutter before...
            Powerup(A::AscendantLight),
            Any(&[
                Powerup(A::HeliacalPower),
                Powerup(A::Sunsetter),
                Powerup(A::SolarWind),
                Powerup(A::ClingGem(2)),
            ]),
        ]),
    },
    Check {
        description: "the locked up time trial",
        location: L::MainUnderbelly,
        index: 1045,
        drop: Drop::Ability(A::Xix),
        key_index: None,
        // need to hit lever at top first
        trial: Some(1028),
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter)]),
    },
    Check {
        description: "behind the locked door",
        location: L::HpSave,
        index: 1047,
        drop: Drop::Ability(A::HeliacalPower),
        key_index: None,
        trial: None,
        locks: Lock::SmallKey,
    },
    Check {
        description: "the note behind the locked door",
        location: L::HpSave,
        index: 895,
        drop: Drop::Note,
        key_index: None,
        trial: None,
        locks: Lock::SmallKey,
    },
    Check {
        description: "the note near the empty bailey entrance",
        location: L::BaileyHole,
        index: 896,
        drop: Drop::Note,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SunGreaves),
            All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
            Powerup(A::ClingGem(6)),
            Powerup(A::AscendantLight),
            Powerup(A::SolarWind),
            All(&[Powerup(A::HeliacalPower), Trick(T::ReverseKick, D::Normal)]),
            All(&[Powerup(A::Sunsetter), Trick(T::Movement, D::Normal)]),
        ]),
    },
    Check {
        description: "on top of the big building",
        location: L::BaileyHole,
        index: 615,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::SunGreaves),
            Powerup(A::Sunsetter),
            Powerup(A::SolarWind),
        ]),
    },
    // Tower Ruins
    Check {
        description: "where cling gem normally is",
        location: L::TowerRuinsKeep,
        index: 155,
        drop: Drop::Ability(A::ClingGem(6)),
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::ClingGem(6)),
            Powerup(A::SunGreaves),
            All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter), Trick(T::Movement, D::Normal)]),
        ]),
    },
    Check {
        description: "atop the tower",
        location: L::TowerRuinsKeep,
        index: 77,
        drop: Drop::BigKey,
        key_index: Some(3),
        trial: None,
        locks: Any(&[
            All(&[
                Powerup(A::ClingGem(2)),
                Any(&[
                    Powerup(A::SunGreaves),
                    All(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::Sunsetter),
                    ]),
                ]),
            ]),
            All(&[
                Powerup(A::SunGreaves),
                Powerup(A::SolarWind),
                Trick(T::Movement, D::Expert),
                Trick(T::OneWall, D::Advanced),
            ])
        ]),
    },
    Check {
        description: "the time trial at the tower entrance",
        location: L::TowerRuinsKeep,
        index: 156,
        drop: Drop::Ability(A::BleedingHeart),
        key_index: None,
        // again again just to get to top of tower
        // need to hit lever tho
        trial: Some(129),
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter)]),
    },
    // Twilight Theatre
    Check {
        description: "on a beam in the corner",
        location: L::PillarRoom,
        index: 1231,
        drop: Drop::Ability(A::AerialFinesse),
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[
              Powerup(A::SunGreaves),
              Any(&[Powerup(A::ClingGem(2)), Powerup(A::SolarWind), Powerup(A::Sunsetter)])
            ]),
            All(&[Powerup(A::SolarWind), Powerup(A::ClingGem(2))]),
            All(&[Powerup(A::Sunsetter), Powerup(A::SunGreaves), Trick(T::SunsetterAbuse, D::Normal), Trick(T::Movement, D::Advanced), Trick(T::OneWall, D::Normal)]),
        ]),
    },
    Check {
        description: "the locked up time trial",
        location: L::PillarRoom,
        index: 1232,
        drop: Drop::Ability(A::Classy),
        key_index: None,
        // need to hit the lever first
        trial: Some(1211),
        locks: All(&[Powerup(A::DreamBreaker), Powerup(A::SolarWind), Powerup(A::SunGreaves), Powerup(A::ClingGem(6)), Powerup(A::Sunsetter)]),
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 1081,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 1084,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "a chair around the table",
        location: L::OtherTheatrePath,
        index: 1085,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the chair next to the books",
        location: L::OtherTheatrePath,
        index: 1086,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling who can eat 20 beans at least",
        location: L::TheatreEntrance,
        index: 1072,
        drop: Drop::Goatling(&[
            "where have [#cf2525](you been)?",
            "three bean casserole? not enough for 1 man. i can eat like, [#cf2525](20 beans), at least.",
            "so get to it. [up, 10rr](please?)"
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling who thought the theatre was safe",
        location: L::TheatreEntrance,
        index: 1074,
        drop: Drop::Goatling(&[
            "I heard that the theatre was still in good condition...",
            "But it seems even this place has been affected."
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 1079,
        drop: Drop::Goatling(&[
            "Ah nuts....I really wanted to see the show today."
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling who really wanted to see the show",
        location: L::TheatreEntrance,
        index: 1080,
        drop: Drop::Goatling(&[
            "Sorry miss, can't let you in.",
            "Theatre's closed until all the haze is gone."
        ]),
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "hiding amid the boxes",
        location: L::MainTheatre,
        index: 912,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Lock::None,
    },
    Check {
        description: "the goatling that will kill again",
        location: L::MainTheatre,
        index: 1071,
        drop: Drop::Goatling(&[
            "please leave me alone?",
            "i will [#cf2525](kill again)"
        ]),
        key_index: None,
        trial: None,
        locks: Any(&[
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower)]),
            Powerup(A::SunGreaves),
            Powerup(A::ClingGem(6))
        ]),
    },
    Check {
        description: "the chair near the courtyard",
        location: L::MainTheatre,
        index: 1082,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks: Any(&[
            Powerup(A::ClingGem(4)),
            All(&[Powerup(A::SolarWind), Powerup(A::HeliacalPower), Powerup(A::SunGreaves)]),
        ]),
    },
    Check {
        description: "the chair in the soul cutter zone",
        location: L::MainTheatre,
        index: 1083,
        drop: Drop::Chair,
        key_index: None,
        trial: None,
        locks:  All(&[
            Powerup(A::Strikebreak), 
            Powerup(A::ClingGem(6)),
            Any(&[
                // Logic for soul cutter route w/o soulcutter
                All(&[
                    Powerup(A::Strikebreak),
                    Powerup(A::SolarWind),
                    Powerup(A::HeliacalPower),
                    Powerup(A::SunGreaves),
                    Powerup(A::Sunsetter),
                    Trick(T::ClingAbuse, D::Expert),
                    Trick(T::OneWall, D::Expert),
                    Trick(T::Movement, D::Insane),
                    Trick(T::Momentum, D::Expert),
                ]),
                //with soul cutter.
                All(&[
                    Powerup(A::SoulCutter),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind)
                    ]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "behind three maximum security cages",
        location: L::MainTheatre,
        index: 871,
        drop: Drop::BigKey,
        key_index: Some(5),
        // there's one gap in the open green room with enemies which is too big
        trial: None,
        locks: All(&[
            Powerup(A::Strikebreak), 
            Powerup(A::ClingGem(6)),
            Any(&[
                // Logic for soul cutter route
                All(&[
                    Powerup(A::Strikebreak),
                    Powerup(A::SolarWind),
                    Powerup(A::HeliacalPower),
                    Powerup(A::SunGreaves),
                    Powerup(A::Sunsetter),
                    Trick(T::ClingAbuse, D::Expert),
                    Trick(T::OneWall, D::Expert),
                    Trick(T::Movement, D::Insane),
                    Trick(T::Momentum, D::Expert),
                ]),
                All(&[
                    Powerup(A::SoulCutter),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind)
                    ]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "where soul cutter normally is",
        location: L::MainTheatre,
        index: 1230,
        drop: Drop::Ability(A::SoulCutter),
        key_index: None,
        trial: None,
        locks: All(&[
            //Absolutely neccassary 
            Powerup(A::Strikebreak),
            
            Any(&[
                //Trick for leaving same way as came in.
                All(&[                    
                    Powerup(A::SolarWind),
                    Trick(T::Movement, D::Expert), // Go under the gate as it closes to drop back down. then use abilities to scale shafts.
                    Powerup(A::SunGreaves),
                    Powerup(A::HeliacalPower),
                    Powerup(A::ClingGem(6)),
                    Powerup(A::Sunsetter),
                    Trick(T::OneWall, D::Expert),
                    Trick(T::Knowledge, D::Advanced), // for going under gate from soulcutter and knowing not being locked and needing to climb.
                ]),
                // Go the soul cutter route out to lever without Soulcutter.
                All(&[
                    Powerup(A::Strikebreak),
                    Powerup(A::SolarWind),
                    Powerup(A::HeliacalPower),
                    Powerup(A::SunGreaves),
                    Powerup(A::Sunsetter),
                    Trick(T::ClingAbuse, D::Expert),
                    Trick(T::OneWall, D::Expert),
                    Trick(T::Movement, D::Insane),
                    Trick(T::Momentum, D::Expert),
                ]),
                // Go out the lever route WITH soul cutter.
                All(&[
                    Powerup(A::SoulCutter),
                    Any(&[
                        Powerup(A::HeliacalPower),
                        Powerup(A::SolarWind)
                    ]),
                ]),
            ]),
        ]),
    },
    Check {
        description: "in the back on a pillar",
        location: L::MainTheatre,
        index: 913,
        drop: Drop::Health,
        key_index: None,
        trial: None,
        locks: Any(&[Powerup(A::SunGreaves), Powerup(A::ClingGem(6))]),
    },
    Check {
        description: "behind the locked door",
        location: L::MainTheatre,
        index: 1233,
        drop: Drop::Ability(A::Empathy),
        key_index: None,
        trial: None,
        locks: All(&[
            Lock::SmallKey,
            Any(&[
                Powerup(A::SunGreaves),
                Powerup(A::ClingGem(2)),
            ]),
        ]),
    },
];
