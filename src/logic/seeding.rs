use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn accessible(
    locks: &[&[Lock]],
    locations: &[Location],
    obtainable: &[Drop],
    app: &crate::Rando,
    ctx: Option<(usize, &Drop)>,
) -> bool {
    if locks.is_empty() {
        return true;
    }
    locks.iter().any(|locks| {
        locks.iter().all(|lock| match lock {
            Lock::Location(loc) => locations.contains(loc),
            Lock::Movement(movement) => movement.iter().any(|ability| {
                ability.iter().all(|ability| {
                    // in the cases where requirements are after obtaining
                    let case = match ctx {
                        Some((356, Drop::Ability(Ability::Slide))) => {
                            [Drop::Ability(Ability::Slide)].as_slice()
                        }
                        Some((267, Drop::Ability(Ability::SunGreaves))) => {
                            [Drop::Ability(Ability::SunGreaves)].as_slice()
                        }
                        Some((393, Drop::Ability(Ability::Strikebreak))) => {
                            [Drop::Ability(Ability::Strikebreak)].as_slice()
                        }
                        Some((392, Drop::Ability(Ability::Sunsetter))) => {
                            [Drop::Ability(Ability::Sunsetter)].as_slice()
                        }
                        Some((104, Drop::Ability(Ability::SolarWind))) => {
                            [Drop::Ability(Ability::SolarWind)].as_slice()
                        }
                        Some((1079, Drop::Ability(Ability::SoulCutter))) => {
                            [Drop::Ability(Ability::SoulCutter)].as_slice()
                        }
                        _ => [].as_slice(),
                    };
                    match ability {
                        Ability::Slide if app.progressive => {
                            obtainable.iter().chain(case).any(|drop| {
                                matches!(
                                    drop,
                                    Drop::Ability(Ability::Slide)
                                        | Drop::Ability(Ability::SolarWind)
                                )
                            })
                        }
                        Ability::SolarWind if app.progressive => {
                            obtainable.contains(&Drop::Ability(Ability::Slide))
                                && obtainable.contains(&Drop::Ability(Ability::SolarWind))
                        }
                        Ability::Strikebreak if app.progressive => {
                            obtainable.iter().chain(case).any(|drop| {
                                matches!(
                                    drop,
                                    Drop::Ability(Ability::Strikebreak)
                                        | Drop::Ability(Ability::SoulCutter)
                                )
                            })
                        }
                        Ability::SoulCutter if app.progressive => {
                            obtainable.contains(&Drop::Ability(Ability::Strikebreak))
                                && obtainable.contains(&Drop::Ability(Ability::SoulCutter))
                        }
                        Ability::ClingGem if app.split_cling => {
                            obtainable
                                .iter()
                                .chain(case)
                                .fold(0, |acc, drop| match drop {
                                    Drop::Ability(Ability::ClingGem) => acc + 1,
                                    _ => acc,
                                })
                                == 3
                        }
                        ability => obtainable.iter().chain(case).any(|drop| match drop {
                            Drop::Ability(a) => a == ability,
                            _ => false,
                        }),
                    }
                })
            }),
            // need to decrement small keys :p
            Lock::SmallKey => obtainable.contains(&Drop::SmallKey),
            Lock::Ending => {
                obtainable
                    .iter()
                    .fold(0, |acc, drop| match matches!(drop, Drop::BigKey) {
                        true => acc + 1,
                        false => acc,
                    })
                    == 5
            }
        })
    })
}

fn possible(checks: &[Check], app: &crate::Rando) -> bool {
    let mut checks = checks.to_vec();
    let mut locations: Vec<Location> = Vec::with_capacity(Location::COUNT);
    let mut locations_len = 0;
    let mut obtainable = Vec::with_capacity(checks.len());
    let mut obtainable_len = 0;
    loop {
        for loc in Location::iter() {
            if !locations.contains(&loc)
                && accessible(loc.locks(), &locations, &obtainable, app, None)
            {
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
                        check.locks,
                        &locations,
                        &obtainable,
                        app,
                        Some((check.index, &check.drop)),
                    ))
                .then_some(i)
            })
            .collect();
        for i in slated {
            obtainable.push(checks.remove(i).drop)
        }
        // if all locations accessible then possible
        if locations.len() == Location::COUNT {
            break true;
        }
        // if no change in location or drop count then impossible
        if locations.len() == locations_len && obtainable.len() == obtainable_len {
            break false;
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
    }
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
        Drop::Ability(_) => app.abilities,
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
        pool.extend([
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1548,
                drop: Drop::Ability(A::HeliacalPower),
                locks: &[&[Lock::Movement(&[
                    &[A::Slide, A::SunGreaves],
                    &[A::Slide, A::HeliacalPower],
                    &[A::SunGreaves, A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Slide, A::SolarWind],
                ])]],
            },
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1554,
                drop: Drop::Ability(A::HeliacalPower),
                locks: &[&[Lock::Movement(&[
                    &[A::Slide, A::SunGreaves],
                    &[A::Slide, A::HeliacalPower],
                    &[A::SunGreaves, A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Slide, A::SolarWind],
                ])]],
            },
            Check {
                description: "where sun greaves normally is",
                location: Location::MainLibrary,
                index: 1560,
                drop: Drop::Ability(A::HeliacalPower),
                locks: &[&[Lock::Movement(&[
                    &[A::Slide, A::SunGreaves],
                    &[A::Slide, A::HeliacalPower],
                    &[A::SunGreaves, A::HeliacalPower],
                    &[A::ClingGem],
                    &[A::Slide, A::SolarWind],
                ])]],
            },
        ]);
    }
    if app.split_cling {
        if let Some(i) = pool
            .iter()
            .position(|check| check.drop == Drop::Ability(Ability::ClingGem))
        {
            pool.remove(i);
        }
        use Ability as A;
        pool.extend([
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuins,
                index: 89,
                drop: Drop::Ability(A::ClingGem),
                locks: &[&[Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::HeliacalPower, A::Sunsetter],
                ])]],
            },
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuins,
                index: 95,
                drop: Drop::Ability(A::ClingGem),
                locks: &[&[Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::HeliacalPower, A::Sunsetter],
                ])]],
            },
            Check {
                description: "where cling gem normally is",
                location: Location::TowerRuins,
                index: 101,
                drop: Drop::Ability(A::ClingGem),
                locks: &[&[Lock::Movement(&[
                    &[A::ClingGem],
                    &[A::SunGreaves],
                    &[A::HeliacalPower, A::Sunsetter],
                ])]],
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
    let overworld: std::collections::BTreeMap<_, _> = loop {
        let mut checks = pool.clone();
        let mut drops: Vec<Drop> = checks.iter().map(|check| check.drop).collect();
        use rand::seq::SliceRandom;
        drops.shuffle(&mut rng);
        for (check, drop) in checks.iter_mut().zip(drops.into_iter()) {
            check.drop = drop
        }
        checks.extend_from_slice(&unrandomised);
        if possible(&checks, app) {
            let mut overworld = std::collections::BTreeMap::<_, Vec<_>>::new();
            for check in checks {
                match overworld.get_mut(check.location.file()) {
                    Some(checks) => checks.push(check),
                    None => {
                        overworld.insert(check.location.file(), vec![check]);
                    }
                }
            }
            break overworld;
        }
    }
    .into_iter()
    .map(|(key, value)| (key, value.into_iter().filter(in_pool).collect()))
    .collect();
    let mut log = std::io::BufWriter::new(
        std::fs::File::create("spoiler_log.txt").map_err(|e| e.to_string())?,
    );
    for val in overworld.values().flatten() {
        use std::io::Write;
        log.write_fmt(format_args!("{:?}\n", val))
            .map_err(|e| e.to_string())?
    }
    crate::writing::write(overworld, app).map_err(|e| e.to_string())
}
