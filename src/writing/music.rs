use super::*;

const PREFIX: &str = "Audio/Music/";

pub fn write(
    app: &crate::Rando,
    music: std::iter::Zip<std::array::IntoIter<Music, 10>, std::array::IntoIter<Music, 10>>,
    pak: &repak::PakReader,
    mod_pak: &mut repak::PakWriter<std::io::BufWriter<std::fs::File>>,
) -> Result<(), Error> {
    std::thread::scope(|thread| -> Result<(), Error> {
        let threads: Vec<_> = music
            .map(|(old, new)| {
                thread.spawn(move || {
                    let mut path = PREFIX.to_string() + old.track() + ".uasset";
                    let mut track = extract(app, pak, &path)?;
                    path = MOD.to_string() + &path;
                    let mut name_map = track.get_name_map();
                    let matched: Vec<_> = name_map
                        .get_ref()
                        .get_name_map_index_list()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, name)| name.contains(old.source()).then_some(i))
                        .collect();
                    for i in matched {
                        let mut name_map = name_map.get_mut();
                        let name = name_map.get_name_reference_mut(i as i32);
                        *name = name.replace(old.source(), new.source());
                    }
                    if let Some(cue) =
                        unreal_asset::cast!(Export, NormalExport, &mut track.asset_data.exports[0])
                    {
                        match cue.properties.iter_mut().find_map(|prop| {
                            match prop
                                .get_name()
                                .get_content(|name| name == "VolumeMultiplier")
                            {
                                true => unreal_asset::cast!(Property, FloatProperty, prop),
                                false => None,
                            }
                        }) {
                            Some(volume) => {
                                volume.value = new.volume().into();
                            }
                            _ => cue.properties.push(Property::FloatProperty(
                                int_property::FloatProperty {
                                    name: name_map.get_mut().add_fname("VolumeMultiplier"),
                                    ancestry: Default::default(),
                                    property_guid: Some(Default::default()),
                                    duplication_index: Default::default(),
                                    value: new.volume().into(),
                                },
                            )),
                        }
                    }
                    let mut asset = std::io::Cursor::new(vec![]);
                    let mut bulk = std::io::Cursor::new(vec![]);
                    track.write_data(&mut asset, Some(&mut bulk))?;
                    Ok::<_, Error>((path, asset, bulk))
                })
            })
            .collect();
        for thread in threads {
            let (path, asset, bulk) = thread.join()??;
            mod_pak.write_file(&path, asset.into_inner())?;
            mod_pak.write_file(&path.replace(".uasset", ".uexp"), bulk.into_inner())?;
        }
        Ok(())
    })?;
    Ok(())
}
