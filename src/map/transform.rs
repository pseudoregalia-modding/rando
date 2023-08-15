use unreal_asset::{
    exports::ExportNormalTrait,
    properties::{struct_property::StructProperty, vector_property::VectorProperty, *},
    types::vector::Vector,
    *,
};

const DEFAULT: Vector<f64> = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub fn get_location<C: std::io::Read + std::io::Seek>(
    index: usize,
    asset: &Asset<C>,
) -> Vector<f64> {
    let Some(transform) = get_transform_index(index, asset) else {
        return DEFAULT
    };
    asset.asset_data.exports[transform]
        .get_normal_export()
        .and_then(|norm| {
            norm.properties.iter().rev().find_map(|prop| {
                if let Property::StructProperty(struc) = prop {
                    if &struc.name == "RelativeLocation" {
                        if let Property::VectorProperty(vec) = &struc.value[0] {
                            return Some(Vector {
                                x: vec.value.x.0,
                                y: vec.value.y.0,
                                z: vec.value.z.0,
                            });
                        }
                    }
                }
                None
            })
        })
        .unwrap_or(DEFAULT)
}

pub fn set_location<C: std::io::Read + std::io::Seek>(
    index: usize,
    asset: &mut Asset<C>,
    new: Vector<f64>,
    offset: (f64, f64, f64),
) {
    let mut name_map = asset.get_name_map();
    let (x, y, z) = offset;
    let Some(transform) = get_transform_index(index, asset) else {
        return
    };
    let Some(norm) = asset.asset_data.exports[transform].get_normal_export_mut() else {
        return
    };
    match norm
        .properties
        .iter_mut()
        .find(|prop| prop.get_name() == "RelativeLocation")
    {
        Some(scale) => {
            if let Property::StructProperty(struc) = scale {
                if let Property::VectorProperty(vec) = &mut struc.value[0] {
                    vec.value.x.0 = new.x + x;
                    vec.value.y.0 = new.y + y;
                    vec.value.z.0 = new.z + z;
                }
            }
        }
        None => {
            let name = name_map.get_mut().add_fname("RelativeLocation");
            let struct_type = Some(name_map.clone_resource().get_mut().add_fname("Vector"));
            norm.properties
                .push(Property::StructProperty(StructProperty {
                    name,
                    ancestry: unversioned::ancestry::Ancestry {
                        ancestry: Vec::new(),
                    },
                    struct_type,
                    struct_guid: None,
                    property_guid: None,
                    duplication_index: 0,
                    serialize_none: true,
                    value: vec![Property::VectorProperty(VectorProperty {
                        name: name_map.get_mut().add_fname("RelativeLocation"),
                        ancestry: unversioned::ancestry::Ancestry {
                            ancestry: Vec::new(),
                        },
                        property_guid: None,
                        duplication_index: 0,
                        value: Vector::new(new.x.into(), new.y.into(), new.z.into()),
                    })],
                }));
        }
    }
}

fn get_transform_index<C: std::io::Read + std::io::Seek>(
    index: usize,
    asset: &Asset<C>,
) -> Option<usize> {
    asset.asset_data.exports[index]
        .get_normal_export()
        .and_then(|norm| {
            // normally these are further back so reversed should be a bit faster
            norm.properties.iter().rev().find_map(|prop| {
                prop.get_name().get_content(|name| match name {
                    // of course this wouldn't be able to be detected if all transforms were left default
                    "RelativeLocation" | "RelativeRotation" | "RelativeScale3D" => Some(index),
                    "RootComponent" => {
                        if let Property::ObjectProperty(obj) = prop {
                            if obj.value.is_export() {
                                return Some(obj.value.index as usize - 1);
                            }
                        }
                        None
                    }
                    _ => None,
                })
            })
        })
}
