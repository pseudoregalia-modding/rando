use unreal_asset::reader::ArchiveTrait;

use super::*;

pub fn write(
    checks: std::collections::HashMap<Locations, Vec<Check>>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
) -> Result<(), Error> {
    // reference so it isn't moved
    let used = &std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(checks.len())));
    std::thread::scope(|thread| -> Result<(), Error> {
        for thread in checks.into_iter().map(
            |(location, checks)| -> Result<std::thread::ScopedJoinHandle<Result<(), Error>>, Error> {
                Ok(thread.spawn(move || {
                    let mut path = format!("{PREFIX}{location}.umap");
                    let mut map = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    let mut name_map = map.get_name_map();
                    for Check { context, drop, .. } in checks {
                        match context {
                            Context::Overworld(name) => {
                                let mut i = map
                                    .asset_data
                                    .exports
                                    .iter()
                                    .position(|ex| ex.get_base_export().object_name == name)
                                    .ok_or(Error::Assumption)?;
                                let class = map
                                    .get_import(map.asset_data.exports[i].get_base_export().class_index)
                                    .map(|import| import.object_name.get_owned_content())
                                    .unwrap_or_default();
                                let mut replace = |actor: usize| -> Result<(), Error> {
                                    // unfortunately i can't share this between threads
                                    let donor = open_slice(
                                        include_bytes!("../collectibles.umap"),
                                        include_bytes!("../collectibles.uexp"),
                                    )?;
                                    delete(i, &mut map);
                                    let insert = map.asset_data.exports.len();
                                    transplant(actor, &mut map, &donor)?;
                                    let loc = get_location(i, &map);
                                    set_location(
                                        insert,
                                        &mut map,
                                        loc,
                                    );
                                    let mut name = name.to_string();
                                    // create unique id to prevent multiple checks being registered as collected
                                    let mut counter: u16 =
                                        match name.rfind(|ch: char| ch.to_digit(10).is_none()) {
                                            Some(index) if index != name.len() - 1 => name
                                                .drain(index + 1..)
                                                .collect::<String>()
                                                .parse()?,
                                            _ => 1,
                                        };
                                    while used.lock()?.contains(&format!("{name}{counter}"))
                                    {
                                        counter += 1;
                                    }
                                    used.lock()?.push(format!("{name}{counter}"));
                                    let norm = &mut map.asset_data.exports[insert]
                                        .get_normal_export_mut()
                                        .ok_or(Error::Assumption)?;
                                    match norm.properties.iter_mut().find_map(|prop| {
                                        cast!(Property, StrProperty, prop)
                                            .filter(|id| id.name == "ID")
                                    }) {
                                        Some(id) => id.value = Some(format!("{name}{counter}")),
                                        None => norm.properties.push(Property::StrProperty(
                                            str_property::StrProperty {
                                                name: name_map.get_mut().add_fname("ID"),
                                                ancestry: Ancestry { ancestry: Vec::new() },
                                                property_guid: None,
                                                duplication_index: 0,
                                                value: Some(format!("{name}{counter}")),
                                            },
                                        )),
                                    }
                                    i = insert;
                                    Ok(())
                                };
                                match &drop {
                                    Drop::Ability(ability) => todo!(),
                                    Drop::SmallKey => todo!(),
                                    Drop::BigKey => todo!(),
                                    Drop::Health => todo!(),
                                }
                            }
                            _ => (),
                        }
                    }
                    map.rebuild_name_map();
                    save(&mut map, mod_pak, &path)?;
                    Ok(())
                }))
            },
        ) {
            thread?.join()??
        }
        Ok(())
    })?;
    Ok(())
}
