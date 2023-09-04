use super::logic::*;
use super::Mod;
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
    #[error("parse: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("locked poisoned counter")]
    CounterPoison,
    #[error("locked poisoned writer")]
    WriterPoison,
    #[error("extracted poisoned writer")]
    InnerMutex(#[from] std::sync::PoisonError<repak::PakWriter<std::io::BufWriter<std::fs::File>>>),
    #[error("some threads are still using writer")]
    InnerArc,
    #[error("failed to strip prefix when writing file to pak")]
    Strip(#[from] std::path::StripPrefixError),
    #[error("thread failed to complete")]
    Thread,
}

macro_rules! stub {
    ($type: ty, $variant: ident) => {
        impl From<$type> for Error {
            fn from(_: $type) -> Self {
                Self::$variant
            }
        }
    };
}

stub!(
    std::sync::Arc<std::sync::Mutex<repak::PakWriter<std::io::BufWriter<std::fs::File>>>>,
    InnerArc
);
stub!(
    std::sync::PoisonError<
        std::sync::MutexGuard<'_, repak::PakWriter<std::io::BufWriter<std::fs::File>>>,
    >,
    WriterPoison
);
stub!(
    std::sync::PoisonError<std::sync::MutexGuard<'_, i32>>,
    CounterPoison
);
stub!(Box<dyn std::any::Any + Send + 'static>, Thread);

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
    let pak = repak::PakReader::new(&mut sync, repak::Version::V11)?;
    let mod_pak = std::sync::Arc::new(std::sync::Mutex::new(repak::PakWriter::new(
        std::io::BufWriter::new(std::fs::File::create(app.pak.join("rando_p.pak"))?),
        repak::Version::V9,
        "../../../".to_string(),
        None,
    )));
    overworld::write(checks, app, &pak, &mod_pak)?;
    Mod::try_unwrap(mod_pak)?.into_inner()?.write_index()?;
    Ok(())
}
