use super::Location as L;

const SPAWNS: [(&'static str, Location); 34] = [
    //Prison starts
    ("gameStart", L::EarlyPrison),
    ("dungeonlowestSave", L::LatePrison),
    ("dungeonWestSave", L::LatePrison),
    ("dungeonSaveNearBoss", L::StrongEyes),
    ("dungeonWest", L::PEntryUnderBelly),
    ("dungeonNorth", L::PEntryTheatre),
    //Castle Starts
    ("dungeon1", L::CsPrisonEntry),
    ("lowerWestSave", L::CsMain),
    ("lowerWest", L::CsTheatreEntryNearPrison),
    ("lowerSouthHigh", L::CsKeepEntryMain),
    ("startGazebo", L::CastleSansaMain),
    ("lowerNorth", L::CsKeepEntryRamp),
    ("exterior1", L::CsBaileyEntry),
    ("lowerMiddle", L::CsKeepClimbEntrance),
    ("lowerEastSave", L::CastleSansaMain),
    ("lower1", L::CsPrisonEntry),
    ("lowerNorthWestTheatre", L::CsOldSoftlockRoom),
    ("lowerEast", L::CsLibraryEntry),
    ("lowerNorthNorthWest", L::CsTheatreEntrance),
    // Library starts
    ("libraryWest", L::MainLibrary),
    ("librarySave", L::MainLibrary),
    ("saveLibraryWest", L::LibSaveNearGreaves),
    // Sansa Keep Starts
    ("upperSouth", L::SkCastleMainEntry),
    ("saveUpperMid", L::SansaKeep),
    ("upperMiddle", L::SkCastleClimbEntry),
    ("upperNorthSave", L::SansaKeep),
    ("upperNorth", L::SkCastleRampEntry),
    ("upperNorthEast", L::SkUnderbellyEntry),
    ("upperSouthWest", L::SkTheatreEntry),
    // Empty Bailey Starts
    ("lower1", L::EbEntryCastle),
    ("exteriorWest", L::EbEntryTheatre),
    ("exteriorSouthSave", L::EmptyBailey),
    ("exteriorSouthEast", L::EbEntryRuins),
    ("exteriorEast", L::EbEntryUnderBelly),
    // Underbelly starts
    ("cavesSouth", L::BaileyHole),
    ("dungeonWest", L::PrisonHole),
    ("cavesWestSave", L::VAscendantLight), // Split AL due to DB reqs for levers / Doors/ mobs
    ("postLightSave", L::VAscendantLight),// same as above.
    ("cavesSouthSave", L::BaileyHole),
    ("cavesBigMiddleStart", L::MainUnderbelly),
    ("cavesEast", L::SansaHole),
    ("cavesWest", L::PrisonHole),
    ("cavesBigSideStart", L::HpSave),
    // Theatre saves
    ("theatreEast", L::ThCastleEntryMain), // Main Entrance / up and over
    ("theatreNorthEastUpper", L::ThKeepEntry), // From Keep
    ("theatreSouthEast", L::ThCastleEntryPillar), // From Castle / pillar
    ("theatreNorthEastLower", L::ThDungeonEntry), // From Dungeon
    ("theatreSaveMain", L::MainTheatre), // Save Crystal
    ("theatreSouthWest", L::ThBaileyEntry), // From Bailey
    
];