use unreal_asset::reader::ArchiveTrait;

use super::*;

pub fn write(
    checks: std::collections::BTreeMap<&'static str, Vec<Check>>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &mut repak::PakWriter<std::io::BufWriter<std::fs::File>>,
) -> Result<(), Error> {
    // reference so it isn't moved
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
                        let mut replace = |actor: usize, goatling: bool| -> Result<(), Error> {
                            delete(index, &mut map);
                            let insert = map.asset_data.exports.len();
                            transplant(actor, &mut map, &donor)?;
                            let mut loc = get_location(index, &map);
                            if goatling {
                                loc.z += 100.0
                            }
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
                                    | Ability::Strikebreak => match app.progressive {
                                        true => 46,
                                        false => 5,
                                    },
                                    Ability::HeliacalPower
                                    | Ability::AerialFinesse
                                    | Ability::Pilgrimage
                                    | Ability::Empathy
                                    | Ability::GoodGraces
                                    | Ability::MartialProwess
                                    | Ability::ClearMind
                                    | Ability::Professional => 30,
                                }, false)?;
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
                            }
                            Drop::SmallKey => {
                                replace(24, false)?;
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                if let Some(i) = norm.properties.iter_mut().position(|prop| matches!(prop, Property::IntProperty(..))){
                                    norm.properties.remove(i);
                                }
                            },
                            Drop::BigKey => {
                                replace(18, false)?;
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
                            Drop::Health if class != "BP_HealthPiece_C" => replace(11, false)?,
                            Drop::Goatling(dialogue) => {
                                if class != "BP_NPC" {
                                    replace(36, true)?;
                                }
                                let dialogue = dialogue.iter().enumerate().map(|(i, line)| Property::TextProperty(str_property::TextProperty {
                                    name: FName::new_dummy(i.to_string(), 0),
                                    property_guid: Some(Default::default()),
                                    culture_invariant_string: Some(line.to_string()),
                                    namespace: Some(String::new()),
                                    table_id: None,
                                    flags: 0,
                                    history_type: str_property::TextHistoryType::None,
                                    value: None,
                                    ancestry: Ancestry { ancestry: vec![] },
                                    duplication_index: 0,
                                })).collect();
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                match norm.properties.iter_mut().find_map(|prop|  unreal_asset::cast!(Property, ArrayProperty, prop)){
                                    Some(text) => text.value = dialogue,
                                    None => {
                                        let name = names.get_mut().add_fname("textWindows");
                                        let array_type = Some(names.get_mut().add_fname("Text"));
                                        norm.properties.push(Property::ArrayProperty(array_property::ArrayProperty {
                                            name,
                                            property_guid: Some(Default::default()),
                                            array_type,
                                            value: dialogue,
                                            ..Default::default()
                                        }))
                                    },
                                }
                            }
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
