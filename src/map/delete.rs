use unreal_asset::{exports::*, types::*, *};

/// delete an actor from a map
pub fn delete<C: std::io::Read + std::io::Seek>(index: usize, map: &mut Asset<C>) {
    let val = PackageIndex::new(index as i32 + 1);
    if let Some(level) = map
        .asset_data
        .exports
        .iter_mut()
        .find_map(|ex| cast!(Export, LevelExport, ex))
    {
        if let Some(i) = level.actors.iter().position(|i| i == &val) {
            level.actors.remove(i);
        }
        if let Some(i) = level
            .get_base_export()
            .create_before_serialization_dependencies
            .iter()
            .position(|i| i == &val)
        {
            level
                .get_base_export_mut()
                .create_before_serialization_dependencies
                .remove(i);
        }
    }
}
