use unreal_asset::engine_version::EngineVersion::VER_UE5_1;

pub fn open(asset: Vec<u8>, bulk: Vec<u8>) -> Result<super::Asset<Vec<u8>>, crate::writing::Error> {
    Ok(unreal_asset::Asset::new(
        std::io::Cursor::new(asset),
        Some(std::io::Cursor::new(bulk)),
        VER_UE5_1,
        None,
    )?)
}

pub fn open_slice<'chain>(
    asset: &'chain [u8],
    bulk: &'chain [u8],
) -> Result<super::Asset<&'chain [u8]>, crate::writing::Error> {
    Ok(unreal_asset::Asset::new(
        std::io::Cursor::new(asset),
        Some(std::io::Cursor::new(bulk)),
        VER_UE5_1,
        None,
    )?)
}
