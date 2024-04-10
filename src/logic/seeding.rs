use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn accessible(
    locks: &Lock,
    locations: &[Location],
    obtainable: &[Drop],
    app: &crate::Rando,
    ctx: Option<(usize, &Drop)>,
) -> bool {
    match locks {
        Lock::None => true,
        Lock::Trick(trick, diff) => match trick {
            Trick::Momentum => &app.momentum >= diff,
            Trick::Movement => &app.movement >= diff,
            Trick::ClingAbuse => &app.cling_abuse >= diff,
            Trick::OneWall => &app.one_wall >= diff,
            Trick::PogoAbuse => &app.pogo_abuse >= diff,
            Trick::ReverseKick => &app.reverse_kick >= diff,
            Trick::SunsetterAbuse => &app.sunsetter_abuse >= diff,
            Trick::Knowledge => &app.knowledge >= diff,
        },
        Lock::Any(locks) => locks
            .iter()
            .any(|lock| accessible(lock, locations, obtainable, app, ctx)),
        Lock::All(locks) => locks
            .iter()
            .all(|lock| accessible(lock, locations, obtainable, app, ctx)),
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(ability) => {
            // in the cases where requirements are after obtaining
            let case = match ctx {
                Some((501, Drop::Ability(Ability::DreamBreaker))) => {
                    [Drop::Ability(Ability::DreamBreaker)].as_slice()
                }
                Some((502, Drop::Ability(Ability::Slide))) => {
                    [Drop::Ability(Ability::Slide)].as_slice()
                }
                Some((324, Drop::Ability(Ability::SunGreaves))) => {
                    [Drop::Ability(Ability::SunGreaves)].as_slice()
                }
                Some((565, Drop::Ability(Ability::Strikebreak))) => {
                    [Drop::Ability(Ability::Strikebreak)].as_slice()
                }
                Some((564, Drop::Ability(Ability::Sunsetter))) => {
                    [Drop::Ability(Ability::Sunsetter)].as_slice()
                }
                Some((148, Drop::Ability(Ability::SolarWind))) => {
                    [Drop::Ability(Ability::SolarWind)].as_slice()
                }
                Some((1230, Drop::Ability(Ability::SoulCutter))) => {
                    [Drop::Ability(Ability::SoulCutter)].as_slice()
                }
                _ => [].as_slice(),
            };
            match ability {
                Ability::Slide if app.progressive => obtainable.iter().chain(case).any(|drop| {
                    matches!(
                        drop,
                        Drop::Ability(Ability::Slide) | Drop::Ability(Ability::SolarWind)
                    )
                }),
                Ability::SolarWind => {
                    obtainable
                        .iter()
                        .chain(case)
                        .filter(|drop| {
                            matches!(
                                drop,
                                Drop::Ability(Ability::Slide) | Drop::Ability(Ability::SolarWind)
                            )
                        })
                        .count()
                        == 2
                }
                Ability::DreamBreaker if app.progressive => {
                    obtainable.iter().chain(case).any(|drop| {
                        matches!(
                            drop,
                            Drop::Ability(Ability::DreamBreaker)
                                | Drop::Ability(Ability::Strikebreak)
                                | Drop::Ability(Ability::SoulCutter)
                        )
                    })
                }
                Ability::Strikebreak if app.progressive => {
                    obtainable
                        .iter()
                        .chain(case)
                        .filter(|drop| {
                            matches!(
                                drop,
                                Drop::Ability(Ability::DreamBreaker)
                                    | Drop::Ability(Ability::Strikebreak)
                                    | Drop::Ability(Ability::SoulCutter)
                            )
                        })
                        .count()
                        == 2
                }
                Ability::Strikebreak => {
                    obtainable
                        .iter()
                        .chain(case)
                        .filter(|drop| {
                            matches!(
                                drop,
                                Drop::Ability(Ability::DreamBreaker)
                                    | Drop::Ability(Ability::Strikebreak)
                            )
                        })
                        .count()
                        == 2
                }
                Ability::SoulCutter => {
                    obtainable
                        .iter()
                        .chain(case)
                        .filter(|drop| {
                            matches!(
                                drop,
                                Drop::Ability(Ability::DreamBreaker)
                                    | Drop::Ability(Ability::Strikebreak)
                                    | Drop::Ability(Ability::SoulCutter)
                            )
                        })
                        .count()
                        == 3
                }
                Ability::ClingGem(req) => {
                    &obtainable
                        .iter()
                        .chain(case)
                        .fold(0, |acc, drop| match drop {
                            Drop::Ability(Ability::ClingGem(count)) => acc + count,
                            _ => acc,
                        })
                        >= req
                }
                Ability::AscendantLight => {
                    obtainable
                        .iter()
                        .chain(case)
                        .filter(|drop| {
                            matches!(
                                drop,
                                Drop::Ability(Ability::DreamBreaker)
                                    | Drop::Ability(Ability::AscendantLight)
                            )
                        })
                        .count()
                        == 2
                }
                Ability::HeliacalPower => obtainable.iter().chain(case).any(|drop| {
                    matches!(
                        drop,
                        Drop::Ability(Ability::HeliacalPower) | Drop::Ability(Ability::SunGreaves)
                    )
                }),
                ability => obtainable.iter().chain(case).any(|drop| match drop {
                    Drop::Ability(a) => a == ability,
                    _ => false,
                }),
            }
        }
        // need to decrement small keys :p
        Lock::SmallKey => {
            obtainable.contains(&Drop::SmallKey)
                && (app.knowledge >= Difficulty::Normal
                    && obtainable.contains(&Drop::Ability(Ability::Sunsetter))
                    || obtainable.contains(&Drop::Ability(Ability::DreamBreaker)))
        }
        Lock::Ending => {
            obtainable
                .iter()
                .fold(0, |acc, drop| match matches!(drop, Drop::BigKey) {
                    true => acc + 1,
                    false => acc,
                })
                == 5
        }
    }
}

fn possible(spawn: Location, checks: &[Check], app: &crate::Rando) -> Option<String> {
    let mut seeding = String::new();
    let mut checks = checks.to_vec();
    let mut locations: Vec<Location> = Vec::with_capacity(Location::COUNT);
    locations.push(spawn);
    let mut locations_len = 1;
    let mut obtainable = Vec::with_capacity(checks.len());
    let mut obtainable_len = 0;
    loop {
        for loc in Location::iter() {
            if !locations.contains(&loc)
                && accessible(&loc.locks(), &locations, &obtainable, app, None)
            {
                seeding.push_str(&format!("{loc:?}\n"));
                locations.push(loc)
            }
        }
        let slated: Vec<_> = checks
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, check)| {
                (locations.contains(&check.location)
                    && accessible(
                        &check.locks,
                        &locations,
                        &obtainable,
                        app,
                        Some((check.index, &check.drop)),
                    ))
                .then_some(i)
            })
            .collect();
        for i in slated {
            seeding.push_str(&format!("{:?}\n", checks[i]));
            obtainable.push(checks.remove(i).drop)
        }
        // if all locations accessible then possible
        if locations.len() == Location::COUNT {
            break Some(seeding);
        }
        // if no change in location or drop count then impossible
        if locations.len() == locations_len && obtainable.len() == obtainable_len {
            break None;
        }
        let heliacals: Vec<_> = obtainable
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, drop)| (drop == &Drop::Ability(Ability::HeliacalPower)).then_some(i))
            .collect();
        if heliacals.len() == 3 {
            for i in heliacals {
                obtainable.remove(i);
            }
            obtainable.push(Drop::Ability(Ability::SunGreaves))
        }

        locations_len = locations.len();
        obtainable_len = obtainable.len();
        seeding.push('\n');
    }
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
        Drop::Ability(_) => match check.trial {
            Some(_) => app.outfits,
            None => app.abilities,
        },
        Drop::SmallKey => app.small_keys,
        Drop::BigKey => app.big_keys,
        Drop::Health => app.health,
        Drop::Goatling(_) => app.goatlings,
        Drop::Note => app.notes,
        Drop::Chair => app.chairs,
    };
    let (mut pool, unrandomised): (Vec<_>, Vec<_>) = CHECKS.into_iter().partition(in_pool);
    if app.split_greaves {
        if let Some(i) = pool
            .iter()
            .position(|check| check.drop == Drop::Ability(Ability::SunGreaves))
        {
            pool.remove(i);
        }
        use Ability as A;
        use Lock::{All, Any, Movement as Powerup};
        pool.extend([
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1679,
                drop: Drop::Ability(A::HeliacalPower),
                trial: None,
                locks: Any(&[
                    All(&[Powerup(A::Slide), Powerup(A::SunGreaves)]),
                    All(&[Powerup(A::Slide), Powerup(A::HeliacalPower)]),
                    All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
                    Powerup(A::ClingGem(2)),
                    All(&[Powerup(A::Slide), Powerup(A::SolarWind)]),
                ]),
            },
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1685,
                drop: Drop::Ability(A::HeliacalPower),
                trial: None,
                locks: Any(&[
                    All(&[Powerup(A::Slide), Powerup(A::SunGreaves)]),
                    All(&[Powerup(A::Slide), Powerup(A::HeliacalPower)]),
                    All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
                    Powerup(A::ClingGem(2)),
                    All(&[Powerup(A::Slide), Powerup(A::SolarWind)]),
                ]),
            },
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1691,
                drop: Drop::Ability(A::HeliacalPower),
                trial: None,
                locks: Any(&[
                    All(&[Powerup(A::Slide), Powerup(A::SunGreaves)]),
                    All(&[Powerup(A::Slide), Powerup(A::HeliacalPower)]),
                    All(&[Powerup(A::SunGreaves), Powerup(A::HeliacalPower)]),
                    Powerup(A::ClingGem(2)),
                    All(&[Powerup(A::Slide), Powerup(A::SolarWind)]),
                ]),
            },
        ]);
    }
    if app.split_cling {
        if let Some(i) = pool
            .iter()
            .position(|check| check.drop == Drop::Ability(Ability::ClingGem(6)))
        {
            pool.remove(i);
        }
        use Ability as A;
        use Lock::{All, Any, Movement as Powerup};
        pool.extend([
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuinsKeep,
                index: 853,
                drop: Drop::Ability(A::ClingGem(2)),
                trial: None,
                locks: Any(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SunGreaves),
                    All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
                ]),
            },
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuinsKeep,
                index: 859,
                drop: Drop::Ability(A::ClingGem(2)),
                trial: None,
                locks: Any(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SunGreaves),
                    All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
                ]),
            },
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuinsKeep,
                index: 865,
                drop: Drop::Ability(A::ClingGem(2)),
                trial: None,
                locks: Any(&[
                    Powerup(A::ClingGem(2)),
                    Powerup(A::SunGreaves),
                    All(&[Powerup(A::HeliacalPower), Powerup(A::Sunsetter)]),
                ]),
            },
        ]);
    }
    if pool.len() <= 1
        || (!app.abilities
            && app.small_keys
            && !app.big_keys
            && !app.health
            && !app.goatlings
            && !app.notes
            && !app.chairs)
        || (!app.abilities
            && !app.small_keys
            && app.big_keys
            && !app.health
            && !app.goatlings
            && !app.notes
            && !app.chairs)
        || (!app.abilities
            && !app.small_keys
            && !app.big_keys
            && app.health
            && !app.goatlings
            && !app.notes
            && !app.chairs)
        || (!app.abilities
            && !app.small_keys
            && !app.big_keys
            && !app.health
            && app.goatlings
            && !app.notes
            && !app.chairs)
        || (!app.abilities
            && !app.small_keys
            && !app.big_keys
            && !app.health
            && !app.goatlings
            && app.notes
            && !app.chairs)
        || (!app.abilities
            && !app.small_keys
            && !app.big_keys
            && !app.health
            && !app.goatlings
            && !app.notes
            && app.chairs)
    {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut spawn;
    let (seeding, overworld) = loop {
        use rand::{seq::SliceRandom, Rng};
        spawn = SPAWNS[match app.spawn {
            true => rng.gen_range(0..SPAWNS.len()),
            false => 0,
        }];
        let mut checks = pool.clone();
        let mut drops: Vec<Drop> = checks.iter().map(|check| check.drop).collect();
        drops.shuffle(&mut rng);
        for (check, drop) in checks.iter_mut().zip(drops.into_iter()) {
            check.drop = drop
        }
        checks.extend_from_slice(&unrandomised);
        if let Some(seeding) = possible(spawn.1, &checks, app) {
            let mut overworld = std::collections::BTreeMap::<_, Vec<_>>::new();
            for check in checks {
                match overworld.get_mut(check.location.file()) {
                    Some(checks) => checks.push(check),
                    None => {
                        overworld.insert(check.location.file(), vec![check]);
                    }
                }
            }
            break (seeding, overworld);
        }
    };
    let overworld: std::collections::BTreeMap<_, _> = overworld
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().filter(in_pool).collect()))
        .collect();
    std::fs::write("seeding.log", seeding).map_err(|e| e.to_string())?;
    let mut spoiler =
        std::io::BufWriter::new(std::fs::File::create("spoiler.log").map_err(|e| e.to_string())?);
    for val in overworld.values().flatten() {
        use std::io::Write;
        spoiler
            .write_fmt(format_args!("{:?}\n", val))
            .map_err(|e| e.to_string())?
    }
    crate::writing::write(spawn, overworld, app).map_err(|e| e.to_string())
}
