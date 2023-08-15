use super::logic::*;
use crate::{io::*, map::*};
use unreal_asset::{
    cast,
    containers::{NameMap, SharedResource},
    exports::*,
    properties::*,
    unversioned::Ancestry,
};

mod cutscenes;
mod overworld;
mod savegames;
mod specific;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unreal_asset: {0}")]
    UnrealAsset(#[from] unreal_asset::Error),
    #[error("unpak: {0}")]
    Unpak(#[from] unpak::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("data was not as expected - you may have an older version of the game")]
    Assumption,
}

pub const MOD: &str = "rando_p/pseudoregalia/Content";

const SAVEGAME: &str = "Player/Logic/FrameWork/BlueFireSaveGame.uasset";

const PREFIX: &str = "Maps/World/";

fn extract(
    app: &crate::Rando,
    pak: &unpak::Pak,
    path: &str,
) -> Result<(unreal_asset::Asset<std::fs::File>, std::path::PathBuf), Error> {
    let loc = app.pak.join(MOD).join(path);
    if path != "Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap" {
        std::fs::create_dir_all(loc.parent().expect("is a file"))?;
        pak.read_to_file(&format!("/Game/BlueFire/{path}"), &loc)?;
        pak.read_to_file(
            &format!(
                "/Game/BlueFire/{}",
                path.replace(".uasset", ".uexp").replace(".umap", ".uexp")
            ),
            loc.with_extension("uexp"),
        )?;
    }
    Ok((open(&loc)?, loc))
}

fn byte_property(
    name: &str,
    enum_ty: &str,
    val: &str,
    name_map: &mut SharedResource<NameMap>,
) -> Property {
    let name = name_map.get_mut().add_fname(name);
    let enum_type = Some(name_map.get_mut().add_fname(enum_ty));
    Property::ByteProperty(int_property::ByteProperty {
        name,
        ancestry: Ancestry {
            ancestry: Vec::new(),
        },
        property_guid: None,
        duplication_index: 0,
        enum_type,
        value: int_property::BytePropertyValue::FName(
            name_map
                .get_mut()
                .add_fname(&format!("{}::NewEnumerator{}", enum_ty, val)),
        ),
    })
}

fn set_byte(
    name: &str,
    enum_type: &str,
    val: &str,
    export: &mut normal_export::NormalExport,
    name_map: &mut SharedResource<NameMap>,
) -> Result<(), Error> {
    match export
        .properties
        .iter_mut()
        .find_map(|prop| cast!(Property, ByteProperty, prop).filter(|byte| byte.name == name))
    {
        Some(byte) => {
            use int_property::BytePropertyValue;
            *cast!(BytePropertyValue, FName, &mut byte.value).ok_or(Error::Assumption)? = name_map
                .get_mut()
                .add_fname(&format!("{}::NewEnumerator{}", enum_type, val))
        }
        None => export
            .properties
            .push(byte_property(name, enum_type, val, name_map)),
    }
    Ok(())
}

pub fn write(data: Data, app: &crate::Rando) -> Result<(), Error> {
    let pak = unpak::Pak::new(
        app.pak.join("Blue Fire-WindowsNoEditor.pak"),
        unpak::Version::FrozenIndex,
    )?;
    // correct the shenanigans in spirit hunter
    let loc = app
        .pak
        .join(MOD)
        .join("Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap");
    std::fs::create_dir_all(loc.parent().expect("is a file"))?;
    pak.read_to_file(
        "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap",
        &loc,
    )?;
    pak.read_to_file(
        "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.uexp",
        loc.with_extension("uexp"),
    )?;
    let mut spirit_hunter = open(&loc)?;
    spirit_hunter.asset_data.exports[440]
        .get_base_export_mut()
        .object_name = spirit_hunter.add_fname("Pickup_A02_SRF2");
    save(&mut spirit_hunter, &loc)?;
    std::thread::scope(|thread| -> Result<(), Error> {
        for thread in [
            thread.spawn(|| overworld::write(data.overworld, app, &pak)),
            thread.spawn(|| cutscenes::write(data.cutscenes, app, &pak)),
            thread.spawn(|| savegames::write(data.savegames, data.shop_emotes, app, &pak)),
            thread.spawn(|| specific::write(data.cases, app, &pak)),
        ] {
            thread.join().unwrap()?
        }
        Ok(())
    })?;
    // change the logo so people know it worked
    let logo = app.pak.join(MOD).join("HUD/Menu/Blue-Fire-Logo.uasset");
    std::fs::create_dir_all(logo.parent().expect("is a file"))?;
    std::fs::write(&logo, include_bytes!("blueprints/logo.uasset"))?;
    std::fs::write(
        logo.with_extension("uexp"),
        include_bytes!("blueprints/logo.uexp"),
    )?;
    // package the mod in the most scuffed way possible
    std::fs::write("UnrealPak.exe", include_bytes!("UnrealPak.exe"))?;
    std::fs::write("pak.bat", include_str!("pak.bat"))?;
    // for some reason calling with rust doesn't work so a batch file will do
    std::process::Command::new("./pak.bat")
        .arg(app.pak.join("rando_p"))
        .output()?;
    Ok(())
}

fn create_hook<C: std::io::Read + std::io::Seek>(
    app: &crate::Rando,
    pak: &unpak::Pak,
    get_hook: impl Fn(&std::path::PathBuf) -> Result<unreal_asset::Asset<C>, Error>,
    drop: &Drop,
    cutscene: &str,
    index: usize,
) -> Result<(), Error> {
    let mut loc = app.pak.join(MOD).join("Libraries");
    std::fs::create_dir_all(&loc)?;
    let new_name = format!("{}_Hook", cutscene.split('/').last().unwrap_or_default());
    loc = loc.join(&new_name).with_extension("uasset");
    let mut hook = get_hook(&loc)?;
    // edit the item given by the kismet bytecode in the hook
    let Export::FunctionExport(function_export::FunctionExport {
        struct_export:
            struct_export::StructExport {
                script_bytecode: Some(bytecode),
                ..
            },
        ..
    }) = &mut hook.asset_data.exports[index]
    else {
        return Err(Error::Assumption);
    };
    use unreal_asset::kismet::*;
    let [KismetExpression::ExLet(item_type), KismetExpression::ExLet(index), KismetExpression::ExLet(amount), KismetExpression::ExLetBool(key_item)] =
        &mut bytecode[0..4]
    else {
        return Err(Error::Assumption);
    };
    let [KismetExpression::ExByteConst(item_type), KismetExpression::ExByteConst(index), KismetExpression::ExIntConst(amount)] = [
        item_type.expression.as_mut(),
        index.expression.as_mut(),
        amount.expression.as_mut(),
    ] else {
        return Err(Error::Assumption);
    };
    item_type.value = drop.as_u8();
    index.value = drop.inner_as_u8();
    amount.value = match &drop {
        Drop::Item(_, amount) => *amount,
        Drop::Ore(amount) => *amount,
        _ => 1,
    };
    *key_item.assignment_expression = match &drop {
        Drop::Item(item, ..) if item.key_item() => KismetExpression::ExTrue(ExTrue::default()),
        _ => KismetExpression::ExFalse(ExFalse::default()),
    };
    let self_refs: Vec<usize> = hook
        .get_name_map()
        .get_ref()
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("hook").then_some(i))
        .collect();
    for i in self_refs {
        let mut map = hook.get_name_map();
        let mut map = map.get_mut();
        let name = map.get_name_reference_mut(i as i32);
        *name = name.replace("hook", &new_name);
    }
    save(&mut hook, loc)?;
    let loc = app.pak.join(MOD).join(cutscene).with_extension("uasset");
    std::fs::create_dir_all(loc.parent().expect("is a file"))?;
    pak.read_to_file(&format!("/Game/BlueFire/{cutscene}.uasset"), &loc)?;
    pak.read_to_file(
        &format!("/Game/BlueFire/{cutscene}.uexp"),
        loc.with_extension("uexp"),
    )?;
    let mut cutscene = open(&loc)?;
    let universal_refs: Vec<usize> = cutscene
        .get_name_map()
        .get_ref()
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("UniversalFunctions").then_some(i))
        .collect();
    for i in universal_refs {
        let mut map = cutscene.get_name_map();
        let mut map = map.get_mut();
        let name = map.get_name_reference_mut(i as i32);
        *name = name.replace("UniversalFunctions", &new_name);
    }
    save(&mut cutscene, &loc)?;
    Ok(())
}

impl Drop {
    pub fn as_shop_entry(
        &self,
        price: i32,
        name_map: &mut SharedResource<NameMap>,
    ) -> Vec<Property> {
        let amount_name = name_map
            .get_mut()
            .add_fname("Amount_6_185C591747EF40A592FB63886FDB4281");
        let resets_name = name_map
            .get_mut()
            .add_fname("Resets_8_E303F5DF4270CCEE83F05F974F3661C9");
        let original_amounts_name = name_map
            .get_mut()
            .add_fname("OriginalAmount_11_58C3C17D426D49A439C0EE85D7E9B6EC");
        let price_name = name_map
            .get_mut()
            .add_fname("Price_26_80A37F3645AE8292A9F311B86094C095");
        use int_property::*;
        [
            byte_property(
                "Item_3_54327288464702F41977D48660F8979E",
                "Items",
                match self {
                    Drop::Item(item, ..) => item.as_ref(),
                    Drop::Ore(..) => Items::KinbankDebitCard.as_ref(),
                    Drop::Duck => Items::Duck.as_ref(),
                    _ => "25",
                },
                name_map,
            ),
            Property::IntProperty(IntProperty {
                name: amount_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: match self {
                    Drop::Item(_, amount) => *amount,
                    Drop::Emote(..) => 0,
                    _ => 1,
                },
            }),
            Property::BoolProperty(BoolProperty {
                name: resets_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: false,
            }),
            Property::IntProperty(IntProperty {
                name: original_amounts_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: match self {
                    Drop::Item(_, amount) => *amount,
                    _ => 1,
                },
            }),
            byte_property(
                "Type_17_9B84CFD04716464F71190CB4CECE0F49",
                "InventoryItemType",
                self.as_ref(),
                name_map,
            ),
            byte_property(
                "Tunic_23_B7D465CA4DCF57F409450789A6DB8590",
                "Tunics",
                if let Drop::Tunic(tunic) = self {
                    tunic.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            byte_property(
                "Weapon_22_F3B61F384438EE8A8193F385AE45F88A",
                "Weapons",
                if let Drop::Weapon(weapon) = self {
                    weapon.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            byte_property(
                "Spirit_21_55691F2E4B399DB3F381209D33BBE30B",
                "Spirits",
                if let Drop::Spirit(spirit) = self {
                    spirit.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            Property::IntProperty(IntProperty {
                name: price_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: if let Drop::Ore(amount) = self {
                    -*amount
                } else {
                    price
                },
            }),
            byte_property(
                "Ability_29_EBF42DD143E9F82EC9303082A50329F0",
                "Abilities",
                if let Drop::Ability(ability) = self {
                    ability.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
        ]
        .to_vec()
    }
}

impl Abilities {
    pub fn as_item(&self) -> Items {
        match self {
            Abilities::DoubleJump => Items::DoubleJump,
            Abilities::Dash => Items::Dash,
            Abilities::Attack => todo!(),
            Abilities::DownSmash => Items::DownSmash,
            Abilities::WallRun => Items::WallRun,
            Abilities::Grind => todo!(),
            Abilities::Sprint => Items::Sprint,
            Abilities::Spell => Items::FireBall,
            Abilities::Block => Items::Shield,
            Abilities::SpinAttack => Items::SpinAttack,
        }
    }
    pub fn savegame_index(&self) -> usize {
        match self {
            Abilities::DoubleJump => 2,
            Abilities::Dash => 1,
            Abilities::Attack => 0,
            Abilities::DownSmash => 5,
            Abilities::WallRun => 3,
            Abilities::Grind => 7,
            Abilities::Sprint => 4,
            Abilities::Spell => 6,
            Abilities::Block => 8,
            Abilities::SpinAttack => 9,
        }
    }
}
