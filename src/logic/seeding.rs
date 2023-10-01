use super::*;
use strum::{EnumCount, IntoEnumIterator};

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
    let (pool, unrandomised): (Vec<_>, Vec<_>) = CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1
        || (!app.abilities && app.small_keys && !app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && app.big_keys && !app.health)
        || (!app.abilities && !app.small_keys && !app.big_keys && app.health)
    {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }

    let mut overworld = std::collections::BTreeMap::new();
    let mut rng = rand::thread_rng();
    // shuffle the possible drops
    use rand::seq::SliceRandom;
    let mut checks = pool.clone();
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    possible.shuffle(&mut rng);
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop;
    }
    for check in checks {
        submit(check, &mut overworld)
    }
    for check in unrandomised.clone() {
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
