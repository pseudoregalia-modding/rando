use unreal_asset::reader::ArchiveTrait;

use super::*;

pub fn write(
    checks: std::collections::HashMap<Locations, Vec<Check>>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
) -> Result<(), Error> {
    // reference so it isn't moved
    let counter = &std::sync::Arc::new(std::sync::Mutex::new(0));
    std::thread::scope(|thread| -> Result<(), Error> {
        for thread in checks.into_iter().map(
            |(location, checks)| -> Result<std::thread::ScopedJoinHandle<Result<(), Error>>, Error> {
                Ok(thread.spawn(move || {
                    let mut path = format!("{PREFIX}{location}.umap");
                    let mut map = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    for Check { name, drop, .. } in checks {
                        let Some(mut i) = map
                            .asset_data
                            .exports
                            .iter()
                            .position(|ex| ex.get_base_export().object_name == name) else {continue};
                        let class = map
                            .get_import(map.asset_data.exports[i].get_base_export().class_index)
                            .map(|import| import.object_name.get_owned_content())
                            .unwrap_or_default();
                        let mut replace = |actor: usize| -> Result<(), Error> {
                            // unfortunately i can't share this between threads
                            let donor = open_slice(
                                include_bytes!("../assets/collectibles.umap"),
                                include_bytes!("../assets/collectibles.uexp"),
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
                            i = insert;
                            Ok(())
                        };
                        match &drop {
                            Drop::Ability(ability) => {
                                if class != "BP_UpgradeBase_C"{
                                    replace(5)?
                                }
                                let ability = map.add_fname(ability.as_ref());
                                // upgradebase is 6 exports long
                                let actor = map.asset_data.exports.len() - 6;
                                let Some(norm) = map.asset_data.exports[actor].get_normal_export_mut() else {continue};
                                let Property::NameProperty(row) = &mut norm.properties[7] else {continue};
                                row.value = ability;
                                let Property::IntProperty(id) = &mut norm.properties[5] else {continue};
                                id.value = *counter.lock()?;
                                // can't use += because the i32 is behind a MutexGuard
                                use std::ops::AddAssign;
                                counter.lock()?.add_assign(1);
                            }
                            Drop::SmallKey if class != "BP_GenericKey_C" => replace(24)?,
                            Drop::BigKey if class != "BP_GenericKey_C" => replace(18)?,
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
