use super::logic::*;
use crate::{io::*, map::*};
use unreal_asset::{exports::*, properties::*};

mod overworld;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unreal_asset: {0}")]
    UnrealAsset(#[from] unreal_asset::Error),
    #[error("repak: {0}")]
    Repak(#[from] repak::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parsing int: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("failed to strip prefix when writing file to pak")]
    Strip(#[from] std::path::StripPrefixError),
    #[error("thread failed to complete")]
    Thread,
}

impl From<Box<dyn std::any::Any + Send + 'static>> for Error {
    fn from(_: Box<dyn std::any::Any + Send + 'static>) -> Self {
        Self::Thread
    }
}

pub const MOD: &str = "pseudoregalia/Content/";

const PREFIX: &str = "Maps/";

fn extract(
    app: &crate::Rando,
    pak: &repak::PakReader,
    path: &str,
) -> Result<super::Asset<Vec<u8>>, Error> {
    open(
        pak.get(&format!("pseudoregalia/Content/{path}"), &mut app.pak()?)?,
        pak.get(
            &format!(
                "pseudoregalia/Content/{}",
                path.replace(".uasset", ".uexp").replace(".umap", ".uexp")
            ),
            &mut app.pak()?,
        )?,
    )
}

pub fn write(
    checks: std::collections::BTreeMap<&'static str, Vec<Check>>,
    app: &crate::Rando,
) -> Result<(), Error> {
    let mut sync = app.pak()?;
    let pak = repak::PakBuilder::new()
        .oodle(|| {
            Ok(|comp_buf, raw_buf| unsafe {
                OodleLZ_Decompress(
                    comp_buf.as_ptr(),
                    comp_buf.len(),
                    raw_buf.as_mut_ptr(),
                    raw_buf.len(),
                    1,
                    1,
                    0,
                    0,
                    0,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    3,
                )
            })
        })
        .reader_with_version(&mut sync, repak::Version::V11)?;
    let mut mod_pak = repak::PakBuilder::new()
        // for some reason it's not loading properly with compression
        // .compression([repak::Compression::Zlib])
        .writer(
            std::io::BufWriter::new(std::fs::File::create(app.pak.join("rando_p.pak"))?),
            repak::Version::V9,
            "../../../".to_string(),
            None,
        );
    overworld::write(checks, app, &pak, &mut mod_pak)?;
    if app.progressive {
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/Progressive.uasset",
            include_bytes!("assets/Progressive.uasset"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/Progressive.uexp",
            include_bytes!("assets/Progressive.uexp"),
        )?;
    }
    if app.split_cling {
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/SplitCling.uasset",
            include_bytes!("assets/SplitCling.uasset"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/SplitCling.uexp",
            include_bytes!("assets/SplitCling.uexp"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/WallRunClingLimit.uasset",
            include_bytes!("assets/WallRunClingLimit.uasset"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/WallRunClingLimit.uexp",
            include_bytes!("assets/WallRunClingLimit.uexp"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/LimitSetter.uasset",
            include_bytes!("assets/LimitSetter.uasset"),
        )?;
        mod_pak.write_file(
            "pseudoregalia/Content/Blueprints/LimitSetter.uexp",
            include_bytes!("assets/LimitSetter.uexp"),
        )?;
    }
    mod_pak.write_index()?;
    Ok(())
}

#[link(name = "oo2core_win64", kind = "static")]
extern "C" {
    fn OodleLZ_Decompress(
        compBuf: *const u8,
        compBufSize: usize,
        rawBuf: *mut u8,
        rawLen: usize,
        fuzzSafe: u32,
        checkCRC: u32,
        verbosity: u32,
        decBufBase: u64,
        decBufSize: usize,
        fpCallback: u64,
        callbackUserData: u64,
        decoderMemory: *mut u8,
        decoderMemorySize: usize,
        threadPhase: u32,
    ) -> i32;
}
