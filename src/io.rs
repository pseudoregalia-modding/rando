use unreal_asset::{engine_version::EngineVersion::VER_UE4_25, Asset};

pub fn open(asset: Vec<u8>, bulk: Vec<u8>) -> Result<super::Asset<Vec<u8>>, crate::writing::Error> {
    Ok(Asset::new(
        std::io::Cursor::new(asset),
        Some(std::io::Cursor::new(bulk)),
        VER_UE4_25,
        None,
    )?)
}

pub fn open_slice<'chain>(
    asset: &'chain [u8],
    bulk: &'chain [u8],
) -> Result<super::Asset<&'chain [u8]>, crate::writing::Error> {
    Ok(Asset::new(
        std::io::Cursor::new(asset),
        Some(std::io::Cursor::new(bulk)),
        VER_UE4_25,
        None,
    )?)
}

pub fn save<C: std::io::Read + std::io::Seek>(
    map: &mut Asset<C>,
    mod_pak: &super::Mod,
    path: &str,
) -> Result<(), crate::writing::Error> {
    let mut asset = std::io::Cursor::new(vec![]);
    let mut bulk = std::io::Cursor::new(vec![]);
    map.write_data(&mut asset, Some(&mut bulk))?;
    mod_pak.lock()?.write_file(path, asset.into_inner())?;
    mod_pak.lock()?.write_file(
        &path.replace(".uasset", ".uexp").replace(".umap", ".uexp"),
        bulk.into_inner(),
    )?;
    Ok(())
}
