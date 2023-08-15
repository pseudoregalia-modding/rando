use super::*;

pub fn write(cutscenes: Vec<Check>, app: &crate::Rando, pak: &unpak::Pak) -> Result<(), Error> {
    std::thread::scope(|thread| -> Result<(), Error> {
        let mut threads = Vec::with_capacity(cutscenes.len());
        for Check { context, drop, .. } in cutscenes {
            threads.push(thread.spawn(move || {
                let Context::Cutscene(cutscene) = context else {
                    return Err(Error::Assumption);
                };
                create_hook(
                    app,
                    pak,
                    |_| {
                        Ok(open_from_bytes(
                            include_bytes!("../blueprints/hook.uasset"),
                            include_bytes!("../blueprints/hook.uexp"),
                        )?)
                    },
                    &drop,
                    cutscene,
                    69,
                )?;
                Ok(())
            }));
        }
        for thread in threads {
            thread.join().unwrap()?;
        }
        Ok(())
    })?;
    Ok(())
}
