use std::{fs::File, io::Cursor, path::Path};
use unreal_asset::{engine_version::EngineVersion::VER_UE4_25, error::Error, Asset};

pub fn open(file: impl AsRef<Path>) -> Result<Asset<File>, Error> {
    Asset::new(
        File::open(&file)?,
        File::open(file.as_ref().with_extension("uexp")).ok(),
        VER_UE4_25,
        None,
    )
}

pub fn open_from_bytes<'chain>(
    asset: &'chain [u8],
    bulk: &'chain [u8],
) -> Result<Asset<Cursor<&'chain [u8]>>, Error> {
    Asset::new(
        Cursor::new(asset),
        Some(Cursor::new(bulk)),
        VER_UE4_25,
        None,
    )
}

pub fn save<C: std::io::Read + std::io::Seek>(
    asset: &mut Asset<C>,
    path: impl AsRef<Path>,
) -> Result<(), Error> {
    asset.write_data(
        &mut File::create(&path)?,
        Some(&mut File::create(path.as_ref().with_extension("uexp"))?),
    )
}
