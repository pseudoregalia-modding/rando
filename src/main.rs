#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pseudoregalia_rando::Rando;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::epaint::Vec2::new(420.0, 315.0)),
            icon_data: Some(eframe::IconData {
                rgba: include_bytes!("assets/sybil.rgba").to_vec(),
                width: 32,
                height: 32,
            }),
            app_id: Some("pseudoregalia-rando".to_string()),
            ..Default::default()
        },
        Box::new(|ctx| Box::new(Rando::new(ctx))),
    )
}
