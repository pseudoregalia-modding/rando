use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn update(
    locks: &[&[Lock]],
    locations: &[Location],
    possible: &mut Vec<Drop>,
    checks: &mut Vec<Check>,
    overworld: &mut std::collections::BTreeMap<&'static str, Vec<Check>>,
) -> bool {
    if locks.is_empty() {
        return true;
    }
    // see if there's any requirements met and what they are
    let current = || {
        possible[0..checks.len()]
            .iter()
            .chain(overworld.values().flatten().map(|check| &check.drop))
    };
    match locks.iter().copied().find(|locks| {
        locks.iter().all(|lock| match lock {
            Lock::Location(loc) => locations.contains(&loc),
            Lock::Movement(movement) => movement.iter().any(|ability| {
                ability
                    .iter()
                    .all(|ability| current().any(|drop| drop == &Drop::Ability(*ability)))
            }),
            // need to decrement small keys :p
            Lock::SmallKey => current().any(|drop| matches!(drop, Drop::SmallKey)),
            Lock::Ending => {
                current().fold(0, |acc, drop| match matches!(drop, Drop::BigKey) {
                    true => acc + 1,
                    false => acc,
                }) >= 5
            }
        })
    }) {
        Some(locks) => {
            for lock in locks {
                // freeze any progression items where they are
                while let Some(i) = match lock {
                    Lock::Location(..) => None,
                    Lock::Movement(..) => possible[0..checks.len()]
                        .iter()
                        // not all abilities allow movement
                        .position(|drop| matches!(drop, Drop::Ability(_))),
                    Lock::SmallKey => possible[0..checks.len()]
                        .iter()
                        .position(|drop| matches!(drop, Drop::SmallKey)),
                    Lock::Ending => possible[0..checks.len()]
                        .iter()
                        .position(|drop| matches!(drop, Drop::BigKey)),
                } {
                    let mut check = checks.remove(i);
                    check.drop = possible.remove(i);
                    submit(check, overworld);
                }
            }
            true
        }
        None => false,
    }
}

fn submit(check: Check, overworld: &mut std::collections::BTreeMap<&'static str, Vec<Check>>) {
    match overworld.get_mut(check.location.as_str()) {
        Some(checks) => checks.push(check),
        None => {
            overworld.insert(check.location.as_str(), vec![check]);
        }
    }
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
        Drop::Ability(_) => app.abilities,
        Drop::SmallKey => app.small_keys,
        Drop::BigKey => app.big_keys,
        Drop::Health => app.health,
    };
    let (mut pool, mut unrandomised): (Vec<_>, Vec<_>) = CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1
        || (!app.abilities && app.small_keys && !app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && !app.big_keys && app.health)
    {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());

    let mut overworld = std::collections::BTreeMap::new();
    let mut locations = Vec::with_capacity(Location::COUNT);
    let mut rng = rand::thread_rng();
    while locations.len() != Location::COUNT || !pool.is_empty() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        checks.shuffle(&mut rng);
        // update accessible locations
        for loc in Location::iter() {
            if !locations.contains(&loc)
                && update(
                    loc.locks(),
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut overworld,
                )
            {
                locations.push(loc)
            }
        }
        // update accessible editable checks
        for i in (0..pool.len()).rev() {
            if locations.contains(&pool[i].location)
                && update(
                    pool[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut overworld,
                )
            {
                checks.push(pool.remove(i))
            }
        }
        // update progression with unrandomised
        for i in (0..unrandomised.len()).rev() {
            if locations.contains(&unrandomised[i].location)
                && update(
                    unrandomised[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut overworld,
                )
            {
                submit(unrandomised.remove(i), &mut overworld)
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    for check in checks {
        submit(check, &mut overworld)
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
