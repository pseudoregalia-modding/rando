use unreal_asset::reader::ArchiveTrait;

use super::*;

const PREFIX: &str = "Maps/";

pub fn write(
    checks: std::collections::BTreeMap<&'static str, Vec<Check>>,
    hints: &[String; 5],
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &mut repak::PakWriter<std::io::BufWriter<std::fs::File>>,
) -> Result<(), Error> {
    std::thread::scope(|thread| -> Result<(), Error> {
        let threads: Vec<_> = checks.into_iter().map(
            |(location, checks)| -> Result<std::thread::ScopedJoinHandle<Result<_, Error>>, Error> {
                Ok(thread.spawn(move || {
                    let mut path = PREFIX.to_string() + location + ".umap";
                    let mut map = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    // unfortunately i can't share this between threads
                    let donor = open_slice(
                        include_bytes!("../assets/drops.umap"),
                        include_bytes!("../assets/drops.uexp"),
                    )?;
                    match location {
                        "ZONE_Dungeon" => transplant(36, &mut map, &donor)?,
                        "Zone_Library" if app.split_greaves => {
                            use unreal_asset::types::vector::Vector;
                            delete(324, &mut map);
                            let mut place = |location: Vector<f64>| -> Result<(), Error> {
                                let insert = map.asset_data.exports.len();
                                transplant(30, &mut map, &donor)?;
                                set_location(insert, &mut map, location);
                                Ok(())
                            };
                            place(Vector::new(-4150.0, 9200.0, -100.0))?;
                            place(Vector::new(-4650.0, 9200.0, -250.0))?;
                            place(Vector::new(-3650.0, 9200.0, -250.0))?;
                        }
                        "Zone_Tower" => {
                            if app.split_cling {
                                use unreal_asset::types::vector::Vector;
                                delete(155, &mut map);
                                let mut place = |location: Vector<f64>| -> Result<(), Error> {
                                    let insert = map.asset_data.exports.len();
                                    transplant(59, &mut map, &donor)?;
                                    set_location(insert, &mut map, location);
                                    Ok(())
                                };
                                place(Vector::new(13350.0, 5750.0, 4150.0))?;
                                place(Vector::new(13350.0, 5250.0, 4150.0))?;
                                place(Vector::new(13350.0, 4750.0, 4150.0))?;
                            }
                            if !matches!(app.hints, crate::Hints::None) {
                                for (i, hint) in hints.iter().enumerate() {
                                    let Some(text) = map.asset_data.exports[i + 72]
                                        .get_normal_export_mut()
                                        .and_then(|norm| {
                                            unreal_asset::cast!(
                                                Property,
                                                ArrayProperty,
                                                &mut norm.properties[6]
                                            )
                                        })
                                        .and_then(|arr| {
                                            unreal_asset::cast!(Property, TextProperty, &mut arr.value[0])
                                        })
                                    else {
                                        continue;
                                    };
                                    text.culture_invariant_string = Some(hint.clone())
                                }
                            }
                        }
                        _ => (),
                    }
                    for Check {
                        mut index,
                        drop,
                        trial,
                        ..
                    } in checks
                    {
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
                            set_location(insert, &mut map, loc);
                            index = insert;
                            Ok(())
                        };
                        match &drop {
                            // we always replace here since the same blueprint can have a different appearance
                            Drop::Ability(ability) => {
                                replace(
                                    match ability {
                                        Ability::ClingGem(_) if app.split_cling => 59,
                                        Ability::DreamBreaker
                                        | Ability::SunGreaves
                                        | Ability::Slide
                                        | Ability::Sunsetter
                                        | Ability::ClingGem(_)
                                        | Ability::AscendantLight
                                        | Ability::SoulCutter
                                        | Ability::Indignation
                                        | Ability::SolarWind
                                        | Ability::Strikebreak => match app.progressive {
                                            true => 46,
                                            false => 5,
                                        },
                                        _ => 30,
                                    },
                                    false,
                                )?;
                                let ability_name = map.add_fname(ability.as_ref());
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {
                                    continue;
                                };
                                match norm
                                    .properties
                                    .iter_mut()
                                    .find_map(|prop| unreal_asset::cast!(Property, NameProperty, prop))
                                {
                                    Some(row) => row.value = ability_name,
                                    None => norm.properties.push(Property::NameProperty(
                                        str_property::NameProperty {
                                            name: names.get_mut().add_fname("rowName"),
                                            ancestry: Default::default(),
                                            property_guid: Some(Default::default()),
                                            duplication_index: Default::default(),
                                            value: ability_name,
                                        },
                                    )),
                                }
                                match norm
                                    .properties
                                    .iter_mut()
                                    .find_map(|prop| unreal_asset::cast!(Property, StructProperty, prop))
                                {
                                    Some(row) => *row = ability.data(names.get_mut(), &mut map.imports),
                                    None => norm.properties.push(Property::StructProperty(
                                        ability.data(names.get_mut(), &mut map.imports),
                                    )),
                                }
                            }
                            Drop::SmallKey => {
                                replace(24, false)?;
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {
                                    continue;
                                };
                                if let Some(i) = norm
                                    .properties
                                    .iter_mut()
                                    .position(|prop| matches!(prop, Property::IntProperty(..)))
                                {
                                    norm.properties.remove(i);
                                }
                            }
                            Drop::BigKey(i) => {
                                replace(18, false)?;
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {
                                    continue;
                                };
                                match norm
                                    .properties
                                    .iter_mut()
                                    .find_map(|prop| unreal_asset::cast!(Property, IntProperty, prop))
                                {
                                    Some(id) => id.value = *i,
                                    None => {
                                        norm.properties
                                            .push(Property::IntProperty(int_property::IntProperty {
                                                name: names.get_mut().add_fname("keyID"),
                                                property_guid: Some(Default::default()),
                                                value: *i,
                                                ..Default::default()
                                            }))
                                    }
                                }
                            }
                            Drop::Health if class != "BP_HealthPiece_C" => replace(11, false)?,
                            Drop::Goatling(dialogue) => {
                                if class != "BP_NPC" {
                                    replace(36, true)?;
                                }
                                let dialogue = dialogue
                                    .iter()
                                    .enumerate()
                                    .map(|(i, line)| {
                                        Property::TextProperty(str_property::TextProperty {
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
                                        })
                                    })
                                    .collect();
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {
                                    continue;
                                };
                                match norm
                                    .properties
                                    .iter_mut()
                                    .find_map(|prop| unreal_asset::cast!(Property, ArrayProperty, prop))
                                {
                                    Some(text) => text.value = dialogue,
                                    None => {
                                        let name = names.get_mut().add_fname("textWindows");
                                        let array_type = Some(names.get_mut().add_fname("Text"));
                                        norm.properties.push(Property::ArrayProperty(
                                            array_property::ArrayProperty {
                                                name,
                                                property_guid: Some(Default::default()),
                                                array_type,
                                                value: dialogue,
                                                ..Default::default()
                                            },
                                        ))
                                    }
                                }
                            }
                            Drop::Note if class != "BP_Note" => replace(52, false)?,
                            Drop::Chair if class != "BP_RestChair" => replace(67, false)?,
                            _ => (),
                        }
                        if let Some(trial) =
                            trial.and_then(|i| map.asset_data.exports[i].get_normal_export_mut())
                        {
                            for prop in trial.properties.iter_mut().rev() {
                                match prop {
                                    Property::ObjectProperty(reward) if reward.name == "rewardRef" => {
                                        reward.value = unreal_asset::types::PackageIndex::new(index as i32 + 1);
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    if app.split_cling {
                        transplant(65, &mut map, &donor)?;
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
            mod_pak.write_file(&path.replace(".umap", ".uexp"), bulk.into_inner())?;
        }
        Ok(())
    })?;
    Ok(())
}
