use super::*;

pub fn write(cases: Vec<Check>, app: &crate::Rando, pak: &unpak::Pak) -> Result<(), Error> {
    for Check { context, drop, .. } in cases {
        let Context::Specific(case, index) = context else {
            return Err(Error::Assumption)?
        };
        create_hook(
            app,
            pak,
            |loc| {
                if !loc.exists() {
                    std::fs::write(
                        loc,
                        match case {
                            Case::Bremur => {
                                include_bytes!("../blueprints/bremur_hook.uasset").as_slice()
                            }
                            Case::Paulale => {
                                include_bytes!("../blueprints/paulale_hook.uasset").as_slice()
                            }
                            Case::Angels => {
                                include_bytes!("../blueprints/angel_hook.uasset").as_slice()
                            }
                            Case::AllVoids => {
                                include_bytes!("../blueprints/player_hook.uasset").as_slice()
                            }
                        },
                    )?;
                    std::fs::write(
                        loc.with_extension("uexp"),
                        match case {
                            Case::Bremur => {
                                include_bytes!("../blueprints/bremur_hook.uexp").as_slice()
                            }
                            Case::Paulale => {
                                include_bytes!("../blueprints/paulale_hook.uexp").as_slice()
                            }
                            Case::Angels => {
                                include_bytes!("../blueprints/angel_hook.uexp").as_slice()
                            }
                            Case::AllVoids => {
                                include_bytes!("../blueprints/player_hook.uexp").as_slice()
                            }
                        },
                    )?;
                }
                Ok(open(loc)?)
            },
            &drop,
            case.as_ref(),
            index,
        )?
    }
    Ok(())
}
