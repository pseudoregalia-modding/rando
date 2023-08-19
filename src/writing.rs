use super::logic::*;
use crate::{io::*, map::*};
use unreal_asset::{cast, exports::*, properties::*, unversioned::Ancestry};

mod overworld;

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

const PREFIX: &str = "Maps/";

fn extract(
    app: &crate::Rando,
    pak: &unpak::Pak,
    path: &str,
) -> Result<(unreal_asset::Asset<std::fs::File>, std::path::PathBuf), Error> {
    let loc = app.pak.join(MOD).join(path);
    std::fs::create_dir_all(loc.parent().expect("is a file"))?;
    pak.read_to_file(&format!("/Game/pseudoregalia/{path}"), &loc)?;
    pak.read_to_file(
        &format!(
            "/Game/pseudoregalia/{}",
            path.replace(".uasset", ".uexp").replace(".umap", ".uexp")
        ),
        loc.with_extension("uexp"),
    )?;
    Ok((open(&loc)?, loc))
}

pub fn write(data: Data, app: &crate::Rando) -> Result<(), Error> {
    let pak = unpak::Pak::new(
        app.pak.join("pseudoregalia-Windows.pak"),
        unpak::Version::Fnv64BugFix,
    )?;
    overworld::write(data.overworld, app, &pak)?;
    // package the mod in the most scuffed way possible
    std::fs::write("UnrealPak.exe", include_bytes!("UnrealPak.exe"))?;
    std::fs::write("pak.bat", include_str!("pak.bat"))?;
    // for some reason calling with rust doesn't work so a batch file will do
    std::process::Command::new("./pak.bat")
        .arg(app.pak.join("rando_p"))
        .output()?;
    Ok(())
}
