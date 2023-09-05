use unreal_asset::reader::ArchiveTrait;

use super::*;

pub fn write(
    checks: std::collections::BTreeMap<&'static str, Vec<Check>>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
) -> Result<(), Error> {
    // reference so it isn't moved
    let abilities = &std::sync::Arc::new(std::sync::Mutex::new(1));
    let big_keys = &std::sync::Arc::new(std::sync::Mutex::new(1));
    std::thread::scope(|thread| -> Result<(), Error> {
        for thread in checks.into_iter().map(
            |(location, checks)| -> Result<std::thread::ScopedJoinHandle<Result<(), Error>>, Error> {
                Ok(thread.spawn(move || {
                    let mut path = PREFIX.to_string() + location + ".umap";
                    let mut map = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    for Check { mut index, drop, .. } in checks {
                        let class = map
                            .get_import(map.asset_data.exports[index].get_base_export().class_index)
                            .map(|import| import.object_name.get_owned_content())
                            .unwrap_or_default();
                        let mut replace = |actor: usize| -> Result<(), Error> {
                            // unfortunately i can't share this between threads
                            let donor = open_slice(
                                include_bytes!("../assets/collectibles.umap"),
                                include_bytes!("../assets/collectibles.uexp"),
                            )?;
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
                        match &drop {
                            Drop::Ability(ability) => {
                                // can't be arsed to check data
                                // if class != "BP_UpgradeBase_C" {
                                    replace(match ability {
                                        Ability::SunGreaves
                                        | Ability::Slide
                                        | Ability::Sunsetter
                                        | Ability::ClingGem
                                        | Ability::AscendantLight
                                        | Ability::SoulCutter
                                        | Ability::Indignation
                                        | Ability::SolarWind
                                        | Ability::Strikebreak
                                        | Ability::HeliacalPower => 5,
                                        Ability::AerialFinesse
                                        | Ability::Pilgrimage
                                        | Ability::Empathy
                                        | Ability::GoodGraces
                                        | Ability::MartialProwess
                                        | Ability::ClearMind
                                        | Ability::Professional => 30,
                                    })?;
                                // }
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
                                    Some(id) => id.value = *abilities.lock()?,
                                    None => norm.properties.push(Property::IntProperty(int_property::IntProperty {
                                        name: names.get_mut().add_fname("ID"),
                                        property_guid: Some(Default::default()),
                                        value: *abilities.lock()?,
                                        ..Default::default()
                                    })),
                                }
                                // can't use += because the i32 is behind a MutexGuard
                                use std::ops::AddAssign;
                                abilities.lock()?.add_assign(1);
                            }
                            Drop::SmallKey => {
                                if class != "BP_GenericKey_C" {
                                    replace(24)?
                                }
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                if let Some(i) = norm.properties.iter_mut().position(|prop| matches!(prop, Property::IntProperty(..))){
                                    norm.properties.remove(i);
                                }
                            },
                            Drop::BigKey => {
                                if class != "BP_GenericKey_C" {
                                    replace(18)?;
                                }
                                let mut names = map.get_name_map();
                                let Some(norm) = map.asset_data.exports[index].get_normal_export_mut() else {continue};
                                match norm.properties.iter_mut().find_map(|prop| unreal_asset::cast!(Property, IntProperty, prop)){
                                    Some(id) => id.value = *big_keys.lock()?,
                                    None => norm.properties.push(Property::IntProperty(int_property::IntProperty {
                                        name: names.get_mut().add_fname("keyID"),
                                        property_guid: Some(Default::default()),
                                        value: *big_keys.lock()?,
                                        ..Default::default()
                                    })),
                                }
                                // same here lol
                                use std::ops::AddAssign;
                                big_keys.lock()?.add_assign(1);
                            },
                            Drop::Health if class != "BP_HealthPiece_C" => replace(11)?,
                            _ => ()
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
