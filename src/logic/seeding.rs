use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn accessible(locks: &[&[Lock]], locations: &[Location], obtainable: &[Drop]) -> bool {
    if locks.is_empty() {
        return true;
    }
    // see if there's any requirements met and what they are
    locks
        .iter()
        .copied()
        .find(|locks| {
            locks.iter().all(|lock| match lock {
                Lock::Location(loc) => locations.contains(&loc),
                Lock::Movement(movement) => movement.iter().any(|ability| {
                    ability.iter().all(|ability| {
                        obtainable
                            .iter()
                            .any(|drop| drop == &Drop::Ability(*ability))
                    })
                }),
                // need to decrement small keys :p
                Lock::SmallKey => obtainable.iter().any(|drop| matches!(drop, Drop::SmallKey)),
                Lock::Ending => {
                    obtainable
                        .iter()
                        .fold(0, |acc, drop| match matches!(drop, Drop::BigKey) {
                            true => acc + 1,
                            false => acc,
                        })
                        >= 5
                }
            })
        })
        .is_some()
}

fn possible(checks: &[Check]) -> bool {
    let mut locations: Vec<Location> = Vec::with_capacity(Location::COUNT);
    let mut checks = checks.to_vec();
    let mut locations_len = 0;
    let mut obtainable = Vec::with_capacity(checks.len());
    let mut obtainable_len = 0;
    loop {
        for loc in Location::iter() {
            if !locations.contains(&loc) {
                if accessible(loc.locks(), &locations, &obtainable) {
                    locations.push(loc)
                }
            }
        }
        let slated: Vec<_> = checks
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, check)| accessible(check.locks, &locations, &obtainable).then_some(i))
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
    };
    let (pool, unrandomised): (Vec<_>, Vec<_>) = CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1
        || (!app.abilities && app.small_keys && !app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && !app.big_keys && app.health)
    {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }

    let mut overworld = std::collections::BTreeMap::<_, Vec<_>>::new();
    let mut rng = rand::thread_rng();
    loop {
        let mut checks = pool.clone();
        let mut drops: Vec<Drop> = checks.iter().map(|check| check.drop).collect();
        use rand::seq::SliceRandom;
        drops.shuffle(&mut rng);
        for (check, drop) in checks.iter_mut().zip(drops.into_iter()) {
            check.drop = drop;
        }
        checks.extend_from_slice(&unrandomised);
        if possible(&checks) {
            for check in checks {
                match overworld.get_mut(check.location.as_str()) {
                    Some(checks) => checks.push(check),
                    None => {
                        overworld.insert(check.location.as_str(), vec![check]);
                    }
                }
            }
            break;
        }
    }
    overworld = overworld
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().filter(in_pool).collect()))
        .collect();
    std::fs::write(
        "spoiler_log.txt",
        format!("{:#?}", overworld.values().flatten().collect::<Vec<_>>(),),
    )
    .unwrap_or_default();
    crate::writing::write(overworld, app).map_err(|e| e.to_string())
}
