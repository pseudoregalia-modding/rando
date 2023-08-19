use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn update(
    locks: &[Lock],
    locations: &[Locations],
    possible: &mut Vec<Drop>,
    checks: &mut Vec<Check>,
    data: &mut Data,
) -> bool {
    // see if there's any requirements met and what they are
    if !locks.iter().all(|lock| match lock {
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(movement) => {
            let both: Vec<_> = possible[0..checks.len()]
                .iter()
                .chain(data.overworld.values().flatten().map(|check| &check.drop))
                .collect();
            movement.iter().fold(true, |acc, ability| {
                acc && both.contains(&&Drop::Ability(*ability))
            })
        }
    }) {
        return false;
    }
    for lock in locks {
        // freeze any progression items where they are
        while let Some(i) = match lock {
            Lock::Location(..) => None,
            Lock::Movement(..) => possible[0..checks.len()]
                .iter()
                // not all abilities allow movement
                .position(|drop| matches!(drop, Drop::Ability(_))),
        } {
            let mut check = checks.remove(i);
            check.drop = possible.remove(i);
            push(check, data);
        }
    }
    true
}

fn push(check: Check, data: &mut Data) {
    match check.context {
        Context::Overworld(..) => match data.overworld.get_mut(&check.location) {
            Some(checks) => checks.push(check),
            None => {
                data.overworld.insert(check.location.clone(), vec![check]);
            }
        },
    }
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
        Drop::Ability(_) => app.abilities,
        Drop::SmallKey => app.small_keys,
        Drop::BigKey => app.big_keys,
        Drop::Health => app.health,
    };
    let (mut pool, mut unrandomised): (Vec<Check>, Vec<Check>) =
        CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1 {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut data = Data {
        overworld: std::collections::HashMap::with_capacity(Locations::COUNT),
    };
    let mut locations = Vec::with_capacity(Locations::COUNT);
    let mut rng = rand::thread_rng();
    while locations.len() != Locations::COUNT && !pool.is_empty() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        checks.shuffle(&mut rng);
        // update accessible locations
        for loc in Locations::iter() {
            if !locations.contains(&loc)
                && loc
                    .locks()
                    .iter()
                    .any(|locks| update(locks, &locations, &mut possible, &mut checks, &mut data))
            {
                locations.push(loc);
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
                    &mut data,
                )
            {
                checks.push(pool.remove(i));
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
                    &mut data,
                )
            {
                push(unrandomised.remove(i), &mut data);
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    for check in checks {
        push(check, &mut data)
    }
    data.overworld = data
        .overworld
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().filter(in_pool).collect()))
        .collect();
    std::fs::write(
        "spoiler_log.txt",
        format!(
            "{:#?}",
            data.overworld.values().flatten().collect::<Vec<_>>(),
        ),
    )
    .unwrap_or_default();
    crate::writing::write(data, app).map_err(|e| e.to_string())
}
