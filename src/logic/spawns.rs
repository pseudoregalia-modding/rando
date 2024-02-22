use super::Location as L;

pub const SPAWNS: [(&str, L); 34] = [
    ("gameStart", L::EarlyPrison),
    ("dungeonlowestSave", L::LatePrison),
    ("dungeonWestSave", L::LatePrison),
    ("dungeonSaveNearBoss", L::StrongEyes),
    ("dungeonWest", L::PrisonHole),
    ("dungeonNorth", L::OtherTheatrePath),
    // not including other side since needs logic
    ("dungeon1", L::CastleSansaMain),
    ("lowerWestSave", L::CastleSansaMain),
    ("lowerSouthHigh", L::CastleSansaMain),
    ("startGazebo", L::CastleSansaMain),
    ("lowerNorth", L::CastleSansaMain),
    ("exterior1", L::CastleSansaMain),
    // same for loading zone in locked room
    ("lowerEastSave", L::CastleSansaMain),
    // same for library entrance and easy theatre entrance
    // need to make area with theatre entrance separate location
    ("libraryWest", L::MainLibrary),
    ("librarySave", L::MainLibrary),
    // same for save near sun greaves
    ("upperSouth", L::SansaKeep),
    // same for locked room transition, underbelly entrance and theatre entrance
    ("saveUpperMid", L::SansaKeep),
    ("upperNorthSave", L::SansaKeep),
    ("upperNorth", L::SansaKeep),
    ("lower1", L::EmptyBailey),
    ("exteriorWest", L::EmptyBailey),
    // same for tower entrance and underbelly entrance
    ("exteriorSouthSave", L::EmptyBailey),
    ("cavesSouth", L::BaileyHole),
    // same for keep entrance and save near heliacal power
    ("dungeonWest", L::PrisonHole),
    ("cavesWestSave", L::MainUnderbelly),
    ("postLightSave", L::MainUnderbelly),
    ("cavesSouthSave", L::MainUnderbelly),
    ("cavesBigMiddleStart", L::MainUnderbelly),
    ("theatreEast", L::MainTheatre),
    ("upperSouthWest", L::MainTheatre),
    ("theatreSouthEast", L::PillarRoom),
    ("theatreNorthEastLower", L::MainTheatre),
    ("theatreSaveMain", L::MainTheatre),
    ("theatreSouthWest", L::PillarRoom),
];
