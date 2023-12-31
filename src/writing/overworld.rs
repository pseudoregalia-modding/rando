use unreal_asset::reader::ArchiveTrait;

use super::*;

pub fn write(
    checks: std::collections::BTreeMap<&'static str, Vec<Check>>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &mut repak::PakWriter<std::io::BufWriter<std::fs::File>>,
) -> Result<(), Error> {
    // reference so it isn't moved
    let abilities = &std::sync::atomic::AtomicI32::new(1);
    let big_keys = &std::sync::atomic::AtomicI32::new(1);
    std::thread::scope(|thread| -> Result<(), Error> {
        let threads: Vec<_> = checks.into_iter().map(
            |(location, checks)| -> Result<std::thread::ScopedJoinHandle<Result<_, Error>>, Error> {
                Ok(thread.spawn(move || {
                    let mut path = PREFIX.to_string() + location + ".umap";
                    let mut map = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    // unfortunately i can't share this between threads
                    let donor = open_slice(
                        include_bytes!("../assets/collectibles.umap"),
                        include_bytes!("../assets/collectibles.uexp"),
                    )?;
                    match location {
                        "ZONE_Dungeon" => transplant(36, &mut map, &donor)?,
                        "Zone_Library" if app.split => {
                            use unreal_asset::types::vector::Vector;
                            delete(267, &mut map);
                            let mut place = |location: Vector<f64>| -> Result<(),Error>{
                                let insert = map.asset_data.exports.len();
                                transplant(30, &mut map, &donor)?;
                                set_location(insert, &mut map, location);
                                Ok(())
                            };
                            place(Vector::new(-4150.0, 9200.0, -100.0))?;
                            place(Vector::new(-4650.0, 9200.0, -250.0))?;
                            place(Vector::new(-3650.0, 9200.0, -250.0))?;
                        },
                        _ => ()
                    }
                    for Check { mut index, drop, .. } in checks {
                        let class = map
                            .get_import(map.asset_data.exports[index].get_base_export().class_index)
                            .map(|import| import.object_name.get_owned_content())
                            .unwrap_or_default();
                        let mut replace = |actor: usize| -> Result<(), Error> {
                            delete(index, &mut map);
                            let insert = map.asset_data.exports.len();
                            transplant(actor, &mut map, &donor)?;
                            let loc = get_location(index, &map);
                            set_location(
                                insert,
                                &mut map,
                                loc,
                            );
                            index = insert;
                            Ok(())
                        };
                        // we always replace here since the same blueprint can have a different appearance
                        match &drop {
                            Drop::Ability(ability) => {
                                replace(match ability {
                                    Ability::SunGreaves
                                    | Ability::Slide
                                    | Ability::Sunsetter
                                    | Ability::ClingGem
                                    | Ability::AscendantLight
                                    | Ability::SoulCutter
                                    | Ability::Indignation
                                    | Ability::SolarWind
                                    | Ability::Strikebreak => 5,
                                    Ability::HeliacalPower
                                    | Ability::AerialFinesse
                                    | Ability::Pilgrimage
                                    | Ability::Empathy
                                    | Ability::GoodGraces
                                    | Ability::MartialProwess
                                    | Ability::ClearMind
                                    | Ability::Professional => 30,
                                })?;
                                let ability_name = map.add_fname(ability.as_ref());
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                match norm.properties.iter_mut().find_map(|prop| unreal_asset::cast!(Property, NameProperty, prop)){
                                    Some(row) => row.value = ability_name,
                                    None => norm.properties.push(Property::NameProperty(str_property::NameProperty {
                                        name: names.get_mut().add_fname("rowName"),
                                        ancestry: Default::default(),
                                        property_guid: Some(Default::default()),
                                        duplication_index: Default::default(),
                                        value: ability_name
                                    })),
                                }
                                match norm.properties.iter_mut().find_map(|prop| unreal_asset::cast!(Property, StructProperty, prop)){
                                    Some(row) => *row = ability.data(names.get_mut(), &mut map.imports),
                                    None => norm.properties.push(Property::StructProperty(ability.data(names.get_mut(), &mut map.imports))),
                                }
                                match norm.properties.iter_mut().find_map(|prop| unreal_asset::cast!(Property, IntProperty, prop)){
                                    Some(id) => id.value = abilities.load(std::sync::atomic::Ordering::Relaxed),
                                    None => norm.properties.push(Property::IntProperty(int_property::IntProperty {
                                        name: names.get_mut().add_fname("ID"),
                                        property_guid: Some(Default::default()),
                                        value: abilities.load(std::sync::atomic::Ordering::Relaxed),
                                        ..Default::default()
                                    })),
                                }
                                // can't use += because the i32 is behind a MutexGuard
                                abilities.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            Drop::SmallKey => {
                                replace(24)?;
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                if let Some(i) = norm.properties.iter_mut().position(|prop| matches!(prop, Property::IntProperty(..))){
                                    norm.properties.remove(i);
                                }
                            },
                            Drop::BigKey => {
                                replace(18)?;
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                match norm.properties.iter_mut().find_map(|prop| unreal_asset::cast!(Property, IntProperty, prop)){
                                    Some(id) => id.value = big_keys.load(std::sync::atomic::Ordering::Relaxed),
                                    None => norm.properties.push(Property::IntProperty(int_property::IntProperty {
                                        name: names.get_mut().add_fname("keyID"),
                                        property_guid: Some(Default::default()),
                                        value: big_keys.load(std::sync::atomic::Ordering::Relaxed),
                                        ..Default::default()
                                    })),
                                }
                                big_keys.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            },
                            Drop::Health if class != "BP_HealthPiece_C" => replace(11)?,
                            _ => ()
                        }
                    }
                    map.rebuild_name_map();
                    let mut asset = std::io::Cursor::new(vec![]);
                    let mut bulk = std::io::Cursor::new(vec![]);
                    map.write_data(&mut asset, Some(&mut bulk))?;
                    Ok((path, asset, bulk))
                }))
            },
        ).collect();
        for thread in threads {
            let (path, asset, bulk) = thread?.join()??;
            mod_pak.write_file(&path, asset.into_inner())?;
            mod_pak.write_file(
                &path.replace(".uasset", ".uexp").replace(".umap", ".uexp"),
                bulk.into_inner(),
            )?;
        }
        Ok(())
    })?;
    Ok(())
}
