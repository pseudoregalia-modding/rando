use super::*;

pub fn write(
    checks: Vec<Check>,
    shop_emotes: Vec<(Shop, usize)>,
    app: &crate::Rando,
    pak: &unpak::Pak,
) -> Result<(), Error> {
    let (mut savegame, savegame_loc) = extract(app, pak, SAVEGAME)?;
    let default = savegame.asset_data.exports[1]
        .get_normal_export_mut()
        .ok_or(Error::Assumption)?;
    if app.dash {
        cast!(Property, StructProperty, &mut default.properties[2])
            .and_then(|inventory| cast!(Property, BoolProperty, &mut inventory.value[1]))
            .ok_or(Error::Assumption)?
            .value = false;
    }
    if app.emotes {
        cast!(Property, ArrayProperty, &mut default.properties[15])
            .ok_or(Error::Assumption)?
            .value
            .clear()
    }
    let mut name_map = savegame.get_name_map();
    for Check { context, drop, .. } in checks {
        match context {
            Context::Shop(shop, index, price) => {
                let name = name_map.get_mut().add_fname(shop.as_ref());
                let struct_type = Some(name_map.get_mut().add_fname("Inventory"));
                savegame.asset_data.exports[1]
                    .get_normal_export_mut()
                    .and_then(|norm| {
                        cast!(Property, ArrayProperty, &mut norm.properties[shop as usize])
                    })
                    .ok_or(Error::Assumption)?
                    .value[index] = Property::StructProperty(struct_property::StructProperty {
                    name,
                    ancestry: Ancestry {
                        ancestry: Vec::new(),
                    },
                    struct_type,
                    struct_guid: None,
                    property_guid: None,
                    duplication_index: 0,
                    serialize_none: true,
                    value: drop.as_shop_entry(price, &mut name_map),
                });
            }
            Context::Starting => {
                fn add_item(
                    savegame: &mut unreal_asset::Asset<std::fs::File>,
                    drop: Drop,
                    name_map: &mut SharedResource<NameMap>,
                ) -> Result<(), Error> {
                    let name = name_map.get_mut().add_fname(match drop {
                        Drop::Item(item, ..) if item.key_item() => {
                            "PassiveInventory_48_636C916F4A37F051CF9B14A1402B4C94"
                        }
                        _ => "Inventory_23_288399C5416269F828550FB7376E7942",
                    });
                    let struct_type = Some(name_map.get_mut().add_fname("Inventory"));
                    savegame.asset_data.exports[1]
                        .get_normal_export_mut()
                        .and_then(|default| {
                            cast!(Property, StructProperty, &mut default.properties[3])
                        })
                        .and_then(|stats| {
                            cast!(
                                Property,
                                ArrayProperty,
                                &mut stats.value[match drop {
                                    Drop::Item(item, ..) if item.key_item() => 7,
                                    _ => 6,
                                }]
                            )
                        })
                        .ok_or(Error::Assumption)?
                        .value
                        .push(Property::StructProperty(struct_property::StructProperty {
                            name,
                            ancestry: Ancestry {
                                ancestry: Vec::new(),
                            },
                            struct_type,
                            struct_guid: None,
                            property_guid: None,
                            duplication_index: 0,
                            serialize_none: true,
                            value: drop.as_shop_entry(0, name_map),
                        }));
                    Ok(())
                }
                match &drop {
                    Drop::Ability(ability) => {
                        add_item(
                            &mut savegame,
                            Drop::Item(ability.as_item(), 1),
                            &mut name_map,
                        )?;
                        savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[2])
                            })
                            .and_then(|abilities| {
                                cast!(
                                    Property,
                                    BoolProperty,
                                    &mut abilities.value[ability.savegame_index()]
                                )
                            })
                            .ok_or(Error::Assumption)?
                            .value = true;
                    }
                    Drop::Emote(emote) => {
                        let emotes = savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, ArrayProperty, &mut default.properties[15])
                            })
                            .ok_or(Error::Assumption)?;
                        emotes.value.push(byte_property(
                            &emotes.value.len().to_string(),
                            "E_Emotes",
                            emote.as_ref(),
                            &mut name_map,
                        ))
                    }
                    Drop::Ore(amount) => {
                        savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[3])
                            })
                            .and_then(|stats| cast!(Property, IntProperty, &mut stats.value[0]))
                            .ok_or(Error::Assumption)?
                            .value += *amount;
                    }
                    Drop::Duck => {
                        add_item(&mut savegame, Drop::Item(Items::Duck, 1), &mut name_map)?
                    }
                    Drop::Item(..) => add_item(&mut savegame, drop, &mut name_map)?,
                    Drop::Spirit(spirit) => {
                        let spirits = savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[14])
                            })
                            .and_then(|equipment| {
                                cast!(Property, ArrayProperty, &mut equipment.value[0])
                            })
                            .ok_or(Error::Assumption)?;
                        spirits.value.push(byte_property(
                            &spirits.value.len().to_string(),
                            "Spirits",
                            spirit.as_ref(),
                            &mut name_map,
                        ))
                    }
                    Drop::Weapon(weapon) => {
                        let weapons = savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[14])
                            })
                            .and_then(|equipment| {
                                cast!(Property, ArrayProperty, &mut equipment.value[4])
                            })
                            .ok_or(Error::Assumption)?;
                        weapons.value.push(byte_property(
                            &weapons.value.len().to_string(),
                            "Weapons",
                            weapon.as_ref(),
                            &mut name_map,
                        ))
                    }
                    Drop::Tunic(tunic) => {
                        let tunics = savegame.asset_data.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[14])
                            })
                            .and_then(|equipment| {
                                cast!(Property, ArrayProperty, &mut equipment.value[2])
                            })
                            .ok_or(Error::Assumption)?;
                        tunics.value.push(byte_property(
                            &tunics.value.len().to_string(),
                            "Tunics",
                            tunic.as_ref(),
                            &mut name_map,
                        ))
                    }
                }
            }
            _ => (),
        }
    }
    // clear out external shop items
    let default = savegame.asset_data.exports[1]
        .get_normal_export_mut()
        .ok_or(Error::Assumption)?;
    for (shopkeep, i) in shop_emotes {
        cast!(
            Property,
            ArrayProperty,
            &mut default.properties[shopkeep as usize]
        )
        .ok_or(Error::Assumption)?
        .value
        .remove(i);
    }
    savegame.rebuild_name_map();
    save(&mut savegame, savegame_loc)?;
    Ok(())
}
