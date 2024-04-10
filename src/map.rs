use unreal_asset::{exports::*, properties::*, types::*, *};

mod delete;
pub use delete::delete;
mod transplant;
pub use transplant::transplant;
mod transform;
pub use transform::*;

/// gets all exports related to the given actor
fn get_actor_exports<C: std::io::Seek + std::io::Read>(
    index: usize,
    asset: &Asset<C>,
    offset: usize,
) -> Vec<Export> {
    // get references to all the actor's children
    let mut child_indexes: Vec<PackageIndex> = asset.asset_data.exports[index]
        .get_base_export()
        .create_before_serialization_dependencies
        .iter()
        .filter(|dep| dep.is_export())
        // dw PackageIndex is just a wrapper around i32 which is cloned by default anyway
        .cloned()
        .collect();
    // add the top-level actor reference
    child_indexes.insert(0, PackageIndex::new(index as i32 + 1));

    // get all the exports from those indexes
    let mut children: Vec<Export> = child_indexes
        .iter()
        .filter_map(|index| asset.get_export(*index))
        // i'm pretty sure i have to clone here so i can modify then insert data
        .cloned()
        .collect();

    let package_offset = (offset + 1) as i32;
    // update export references to what they will be once added
    for (i, child_index) in child_indexes.into_iter().enumerate() {
        for child in children.iter_mut() {
            on_export_refs(child, |index| {
                if index == &child_index {
                    index.index = package_offset + i as i32;
                }
            });
        }
    }
    children
}

/// creates and assigns a unique name
fn give_unique_name<C: std::io::Seek + std::io::Read>(
    orig: &mut FName,
    asset: &mut Asset<C>,
) -> Result<(), crate::writing::Error> {
    // for the cases where the number is unnecessary

    if orig
        .get_content(|name| asset.search_name_reference(name))
        .is_none()
    {
        *orig = orig.get_content(|name| asset.add_fname(name));
        return Ok(());
    }
    let mut name = orig.get_owned_content();
    let mut counter: u16 = match name.rfind(|ch: char| ch.to_digit(10).is_none()) {
        Some(index) if index != name.len() - 1 => {
            name.drain(index + 1..).collect::<String>().parse()?
        }
        _ => 1,
    };
    while asset
        .search_name_reference(&format!("{name}{counter}"))
        .is_some()
    {
        counter += 1;
    }
    Ok(*orig = asset.add_fname(&format!("{name}{counter}")))
}

/// on all possible export references
fn on_export_refs(export: &mut Export, mut func: impl FnMut(&mut PackageIndex)) {
    if let Some(norm) = export.get_normal_export_mut() {
        for prop in norm.properties.iter_mut() {
            on_props(prop, &mut func);
        }
    }
    let export = export.get_base_export_mut();
    export
        .create_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .create_before_serialization_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .serialization_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    func(&mut export.outer_index);
}

/// on all of an export's possible references to imports
fn on_import_refs(export: &mut Export, mut func: impl FnMut(&mut PackageIndex)) {
    if let Some(norm) = export.get_normal_export_mut() {
        for prop in norm.properties.iter_mut() {
            on_props(prop, &mut func);
        }
    }
    let export = export.get_base_export_mut();
    func(&mut export.class_index);
    func(&mut export.template_index);
    // not serialization_before_serialization because only the first few map exports have those
    export
        .serialization_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .create_before_serialization_dependencies
        .iter_mut()
        .for_each(&mut func);
}

/// on any possible references stashed away in properties
fn on_props(prop: &mut Property, func: &mut impl FnMut(&mut PackageIndex)) {
    match prop {
        Property::ObjectProperty(obj) => {
            func(&mut obj.value);
        }
        Property::ArrayProperty(arr) => {
            for entry in arr.value.iter_mut() {
                on_props(entry, func);
            }
        }
        Property::MapProperty(map) => {
            for val in map.value.values_mut() {
                on_props(val, func);
            }
        }
        Property::SetProperty(set) => {
            for entry in set.value.value.iter_mut() {
                on_props(entry, func);
            }
            for entry in set.removed_items.value.iter_mut() {
                on_props(entry, func);
            }
        }
        Property::DelegateProperty(del) => func(&mut del.value.object),
        Property::MulticastDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::MulticastSparseDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::MulticastInlineDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::StructProperty(struc) => {
            for entry in struc.value.iter_mut() {
                on_props(entry, func);
            }
        }
        _ => (),
    }
}
