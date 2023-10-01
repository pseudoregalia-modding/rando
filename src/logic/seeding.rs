use super::*;

fn possible(checks: &[Check]) -> bool {
    true
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
