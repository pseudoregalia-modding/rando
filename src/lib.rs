use eframe::egui;

mod io;
mod logic;
mod map;
mod writing;

type Asset<T> = unreal_asset::Asset<std::io::Cursor<T>>;

pub struct Rando {
    notifs: egui_modal::Modal,
    pak: std::path::PathBuf,
    pak_str: String,
    abilities: bool,
    small_keys: bool,
    big_keys: bool,
    health: bool,
    split: bool,
}

impl Rando {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        let get_bool = |key: &str| -> bool {
            ctx.storage
                .map(|storage| {
                    storage
                        .get_string(key)
                        .unwrap_or_default()
                        .parse()
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        };

        let mut font = egui::FontDefinitions::default();
        font.font_data.insert(
            "alittlepot".to_string(),
            egui::FontData::from_static(include_bytes!("assets/alittlepot.ttf")),
        );
        font.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "alittlepot".to_string());
        ctx.egui_ctx.set_fonts(font);
        ctx.egui_ctx.set_pixels_per_point(2.0);

        let notifs = egui_modal::Modal::new(&ctx.egui_ctx, "dialog");

        let pak = match ctx.storage.and_then(|storage| storage.get_string("pak")) {
            Some(path) => path.into(),
            None => loop {
                if let Some(pak) = ask_game_path() {
                    break pak;
                }
            },
        };
        let pak_str = get_pak_str(&pak);

        Self {
            notifs,
            pak,
            pak_str,
            abilities: get_bool("abilities"),
            small_keys: get_bool("small keys"),
            big_keys: get_bool("big keys"),
            health: get_bool("health"),
            split: get_bool("split"),
        }
    }
    fn pak(&self) -> Result<std::io::BufReader<std::fs::File>, std::io::Error> {
        std::fs::File::open(self.pak.join("pseudoregalia-Windows.pak")).map(std::io::BufReader::new)
    }
}

fn ask_game_path() -> Option<std::path::PathBuf> {
    let path = rfd::FileDialog::new()
        .set_title("Select where you have pseudoregalia installed (e.g C:/Program Files (x86)/Steam/steamapps/common/pseudoregalia)")
        .pick_folder()?;
    (path.ends_with("Pseudoregalia")
        && !path.ends_with("pseudoregalia/pseudoregalia")
        && path.join("pseudoregalia/Content/Paks").exists())
    .then(|| path.join("pseudoregalia\\Content\\Paks"))
}

fn get_pak_str(pak: &std::path::Path) -> String {
    let mut pak_str: String = pak.to_str().unwrap_or_default().chars().rev().collect();
    pak_str.truncate(75);
    pak_str = "...".to_string() + &pak_str.chars().rev().collect::<String>();
    pak_str
}

macro_rules! notify {
    ($self:expr, $result: expr, $message: literal) => {
        match $result {
            Ok(..) => $self.notifs.open_dialog(
                Some("success"),
                Some($message),
                Some(egui_modal::Icon::Success),
            ),
            Err(e) => $self
                .notifs
                .open_dialog(Some("owo"), Some(e), Some(egui_modal::Icon::Error)),
        }
    };
}

impl eframe::App for Rando {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("pseudoregalia rando").size(40.0));
                ui.label(egui::RichText::new("by spuds").italics().size(15.0));
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(&self.pak_str);
                if ui.button("...").clicked() {
                    if let Some(pak) = ask_game_path() {
                        self.pak_str = get_pak_str(&pak);
                        self.pak = pak
                    } else {
                        self.notifs.open_dialog(
                            Some(":/"),
                            Some("that isn't a valid pseudoregalia install location"),
                            Some(egui_modal::Icon::Warning),
                        )
                    }
                }
            });
            ui.columns(3, |ui| {
                if ui[0].checkbox(&mut self.abilities, "Abilities").changed() && !self.abilities {
                    self.split = false
                }
                ui[0].checkbox(&mut self.health, "Health");
                ui[1].checkbox(&mut self.small_keys, "Small keys");
                ui[1].checkbox(&mut self.big_keys, "Big keys");
                ui[2].add_enabled(
                    self.abilities,
                    egui::Checkbox::new(&mut self.split, "Split greaves"),
                );
                ui[2].set_enabled(false);
                ui[2].checkbox(&mut false, "Goatlings")
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = ui.fonts(|fonts| {
                    fonts.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), ' ')
                });
                ui.label("chat about the rando on");
                ui.hyperlink_to("discord", "http://discord.gg/Mny2HfNMrT");
                ui.label("and share");
                if ui.link("rando_p.pak").clicked() {
                    notify!(
                        self,
                        std::process::Command::new("explorer",)
                            .arg(&self.pak)
                            .spawn(),
                        "share and put it in the same folder"
                    )
                }
                ui.label("to race!")
            });
            ui.vertical_centered_justified(|ui| {
                if ui.button("uninstall seed").clicked() {
                    notify!(
                        self,
                        std::fs::remove_file(self.pak.join("rando_p.pak")),
                        "randomness has been removed from the game"
                    )
                }
                if ui.button("launch pseudoregalia").clicked() {
                    notify!(
                        self,
                        std::process::Command::new(
                            self.pak
                                .join("../../Binaries/Win64/pseudoregalia-Win64-Shipping.exe")
                        )
                        .spawn(),
                        "game found and launched successfully"
                    )
                }
                if ui
                    .button(egui::RichText::new("generate and install seed").size(33.0))
                    .clicked()
                {
                    std::fs::remove_dir_all(self.pak.join("rando_p")).unwrap_or_default();
                    notify!(
                        self,
                        logic::randomise(self),
                        "seed has been generated, written and installed"
                    );
                    std::fs::remove_dir_all(self.pak.join("rando_p")).unwrap_or_default();
                }
            });
            self.notifs.show_dialog();
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("pak", self.pak.to_str().unwrap_or_default().to_string());
        storage.set_string("abilities", self.abilities.to_string());
        storage.set_string("small keys", self.small_keys.to_string());
        storage.set_string("big keys", self.big_keys.to_string());
        storage.set_string("health", self.health.to_string());
        storage.set_string("split", self.split.to_string());
    }
}
