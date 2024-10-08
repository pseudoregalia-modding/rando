use eframe::egui;

mod io;
mod logic;
mod map;
mod viewer;
mod writing;

type Asset<T> = unreal_asset::Asset<std::io::Cursor<T>>;

#[derive(strum::EnumString, strum::AsRefStr, strum::EnumIter, strum::Display, Default, PartialEq, Clone, Copy)]
enum Hints {
    #[default]
    None,
    Location,
    Description
}

pub struct Rando {
    notifs: egui_modal::Modal,
    credits: egui_modal::Modal,
    faq: egui_modal::Modal,
    tricks: egui_modal::Modal,
    viewer: egui_modal::Modal,
    pak: std::path::PathBuf,
    pak_str: String,
    abilities: bool,
    aspects: bool,
    outfits: bool,
    small_keys: bool,
    big_keys: bool,
    health: bool,
    split_greaves: bool,
    progressive: bool,
    goatlings: bool,
    notes: bool,
    chairs: bool,
    split_cling: bool,
    spawn: bool,
    music: bool,
    hints: Hints,
    momentum: logic::Difficulty,
    one_wall: logic::Difficulty,
    reverse_kick: logic::Difficulty,
    sunsetter_abuse: logic::Difficulty,
    pogo_abuse: logic::Difficulty,
    movement: logic::Difficulty,
    cling_abuse: logic::Difficulty,
    knowledge: logic::Difficulty,
    selected: viewer::Node,
    area: viewer::Area,
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

        let get_difficulty = |key: &str| -> logic::Difficulty {
            use std::str::FromStr;
            logic::Difficulty::from_str(
                &ctx.storage
                    .map(|storage| storage.get_string(key).unwrap_or_default())
                    .unwrap_or_default(),
            )
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

        let notifs = egui_modal::Modal::new(&ctx.egui_ctx, "dialog");

        let pak = match ctx.storage.and_then(|storage| storage.get_string("pak")) {
            Some(path) => path.into(),
            None => match ask_game_path() {
                Some(pak) => pak,
                None => {
                    rfd::MessageDialog::new()
                        .set_title("owo")
                        .set_description("no valid game path provided")
                        .show();
                    std::process::exit(0);
                }
            },
        };
        let pak_str = get_pak_str(&pak);

        Self {
            notifs,
            credits: egui_modal::Modal::new(&ctx.egui_ctx, "credits"),
            faq: egui_modal::Modal::new(&ctx.egui_ctx, "faq"),
            tricks: egui_modal::Modal::new(&ctx.egui_ctx, "trick"),
            viewer: egui_modal::Modal::new(&ctx.egui_ctx, "viewer"),
            pak,
            pak_str,
            abilities: get_bool("abilities"),
            aspects: get_bool("aspects"),
            outfits: get_bool("outfits"),
            small_keys: get_bool("small keys"),
            big_keys: get_bool("big keys"),
            health: get_bool("health"),
            split_greaves: get_bool("split greaves"),
            progressive: get_bool("progressive"),
            goatlings: get_bool("goatlings"),
            notes: get_bool("notes"),
            chairs: get_bool("chairs"),
            split_cling: get_bool("split cling"),
            spawn: get_bool("spawn"),
            music: get_bool("music"),
            hints: {
                use std::str::FromStr;
                Hints::from_str(
                    &ctx.storage
                        .map(|storage| storage.get_string("hints").unwrap_or_default())
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
            },
            momentum: get_difficulty("momentum"),
            one_wall: get_difficulty("one wall"),
            reverse_kick: get_difficulty("reverse kick"),
            sunsetter_abuse: get_difficulty("sunsetter abuse"),
            pogo_abuse: get_difficulty("pogo abuse"),
            movement: get_difficulty("movement"),
            cling_abuse: get_difficulty("cling abuse"),
            knowledge: get_difficulty("knowledge"),
            selected: viewer::Node::Location(logic::Location::EarlyPrison),
            area: viewer::Area::Dungeon,
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
    path.join("pseudoregalia/Content/Paks")
        .exists()
        .then(|| path.join("pseudoregalia\\Content\\Paks"))
}

fn get_pak_str(pak: &std::path::Path) -> String {
    let mut pak_str: String = pak
        .to_str()
        .unwrap_or_default()
        .replace('\\', "/")
        .chars()
        .rev()
        .collect();
    pak_str.truncate(75);
    pak_str = "...".to_string() + &pak_str.chars().rev().collect::<String>();
    pak_str
}

macro_rules! notify {
    ($self:expr, $result: expr, $message: literal) => {
        match $result {
            Ok(..) => $self
                .notifs
                .dialog()
                .with_title("success")
                .with_body($message)
                .with_icon(egui_modal::Icon::Success)
                .open(),
            Err(e) => $self
                .notifs
                .dialog()
                .with_title("owo")
                .with_body(e)
                .with_icon(egui_modal::Icon::Error)
                .open(),
        }
    };
}

impl eframe::App for Rando {
    fn update(&mut self, ctx: &eframe::egui::Context, _s: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("pseudoregalia rando").size(40.0));
                if ui.button("credits").clicked() {
                    self.credits.open()
                }
                if ui.button("faq").clicked() {
                    self.faq.open()
                }
            });
            self.credits.show(|ui| {
                ui.label("coding, reverse engineering, initial logic and logic viewer by spuds");
                ui.label("trick levels and logic overhauls by MeriKatt");
                ui.with_layout(
                    egui::Layout::default()
                        .with_cross_justify(true)
                        .with_cross_align(egui::Align::Center),
                    |ui| self.credits.button(ui, "close"),
                );
            });
            self.faq.show(|ui| {
                ui.spacing_mut().item_spacing.x = ui.fonts(|fonts| {
                    fonts.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), ' ')
                });
                ui.collapsing("where can i talk with people about the rando?", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("you can chat about the rando on the");
                        ui.hyperlink_to("pseudoregalia discord", "http://discord.gg/Mny2HfNMrT");
                    })
                });
                ui.collapsing("how can i share seeds/race people?", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("you can share");
                        if ui.link("rando_p.pak").clicked() {
                            notify!(
                                self,
                                std::process::Command::new(
                                    #[cfg(target_os = "windows")]
                                    "explorer",
                                    #[cfg(target_os = "linux")]
                                    "xdg-open",
                                )
                                .arg(&self.pak)
                                .spawn(),
                                "share and put it in the same folder"
                            )
                        }
                        ui.label("to share/race the seed")
                    })
                });
                ui.with_layout(
                    egui::Layout::default()
                        .with_cross_justify(true)
                        .with_cross_align(egui::Align::Center),
                    |ui| self.faq.button(ui, "close"),
                );
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(&self.pak_str);
                if ui.button("...").clicked() {
                    if let Some(pak) = ask_game_path() {
                        self.pak_str = get_pak_str(&pak);
                        self.pak = pak
                    } else {
                        self.notifs
                            .dialog()
                            .with_title("owo")
                            .with_body("that isn't a valid pseudoregalia install location")
                            .with_icon(egui_modal::Icon::Warning)
                            .open();
                    }
                }
            });
            ui.columns(4, |ui| {
                if ui[0].checkbox(&mut self.abilities, "Abilities").changed() && !self.abilities {
                    self.split_greaves = false;
                    self.split_cling = false;
                    self.progressive = false;
                }
                ui[0].checkbox(&mut self.aspects, "Aspects");
                ui[0].checkbox(&mut self.small_keys, "Small keys");
                ui[0].checkbox(&mut self.big_keys, "Big keys");
                egui::ComboBox::from_id_source("hints")
                    .selected_text(self.hints.as_ref())
                    .show_ui(&mut ui[0], |ui| {
                        use strum::IntoEnumIterator;
                        for hint in Hints::iter() {
                            ui.selectable_value(&mut self.hints, hint, hint.to_string());
                        }
                    });
                
                if ui[1].checkbox(&mut self.outfits, "Outfits").clicked() && self.outfits {
                    self.notifs
                        .dialog()
                        .with_title("little bug")
                        .with_body("currently goatlings don't spawn on time trial completion\nannoying but not game-breaking\nsorry for the inconvenience")
                        .with_icon(egui_modal::Icon::Warning)
                        .open();
                }
                ui[1].checkbox(&mut self.health, "Health");
                ui[1].checkbox(&mut self.goatlings, "Goatlings");
                ui[1].checkbox(&mut self.spawn, "Spawn");
                ui[1].label(egui::RichText::new("Major Key Hints").color(ui[1].style().visuals.widgets.inactive.text_color()));
                
                ui[2].checkbox(&mut self.notes, "Notes");
                ui[2].checkbox(&mut self.chairs, "Chairs");
                ui[2].add_enabled(false, egui::Checkbox::new(&mut false, "Enemies?"));
                ui[2].add_enabled(false, egui::Checkbox::new(&mut false, "Levers?"));
                ui[2].checkbox(&mut self.music, "Music");
                
                ui[3].add_enabled(
                    self.abilities,
                    egui::Checkbox::new(&mut self.split_greaves, "Split greaves"),
                );
                ui[3].add_enabled(
                    self.abilities,
                    egui::Checkbox::new(&mut self.split_cling, "Split cling"),
                );
                ui[3].add_enabled(
                    self.abilities,
                    egui::Checkbox::new(&mut self.progressive, "Progressive items"),
                );
                ui[3].add_enabled(false, egui::Checkbox::new(&mut false, "Transitions?"));
            });
            ui.vertical_centered_justified(|ui| {
                if ui
                    .button(egui::RichText::new("logic viewer").size(25.0))
                    .clicked()
                {
                    self.viewer.open()
                }
                self.viewer.show(|ui| {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("area")
                            .selected_text(self.area.as_ref())
                            .show_ui(ui, |ui| {
                                use strum::IntoEnumIterator;
                                for diff in viewer::Area::iter() {
                                    if ui
                                        .selectable_value(&mut self.area, diff, diff.to_string())
                                        .clicked()
                                    {
                                        self.selected =
                                            viewer::Node::Location(self.area.rooms()[0]);
                                    }
                                }
                            });
                        egui::ComboBox::from_id_source("node")
                            .selected_text(match &self.selected {
                                viewer::Node::Location(loc) => format!("{loc:?}"),
                                viewer::Node::Check(check) => check.description.into(),
                            })
                            .show_ui(ui, |ui| {
                                let rooms = self.area.rooms();
                                for loc in rooms.iter() {
                                    ui.selectable_value(
                                        &mut self.selected,
                                        viewer::Node::Location(*loc),
                                        format!("{loc:?}"),
                                    );
                                }
                                // use own version of selectable value to prevent unnecessary clones
                                for check in logic::CHECKS
                                    .iter()
                                    .filter(|check| rooms.contains(&check.location))
                                {
                                    let equal = match &self.selected {
                                        viewer::Node::Location(_) => false,
                                        viewer::Node::Check(sel) => sel == check,
                                    };
                                    if ui.selectable_label(equal, check.description).clicked()
                                        && !equal
                                    {
                                        self.selected = viewer::Node::Check(check.clone())
                                    }
                                }
                            });
                    });
                    fn collapsing(ui: &mut egui::Ui, text: &str, locks: &[logic::Lock], id: usize) {
                        egui::CollapsingHeader::new(text)
                            .id_source(id)
                            .show(ui, |ui| {
                                for (id, lock) in locks.iter().enumerate() {
                                    show(ui, lock, id)
                                }
                            });
                    }
                    fn show(ui: &mut egui::Ui, locks: &logic::Lock, id: usize) {
                        match locks {
                            logic::Lock::None => (),
                            logic::Lock::Any(locks) => collapsing(ui, "any of", locks, id),
                            logic::Lock::All(locks) => collapsing(ui, "all of", locks, id),
                            logic::Lock::Trick(trick, diff) => {
                                ui.label(match trick {
                                    logic::Trick::Momentum => {
                                        format!("{:?} momentum conservation", diff)
                                    }
                                    logic::Trick::Movement => format!("{:?} movement", diff),
                                    logic::Trick::ClingAbuse => {
                                        format!("{:?} cling abuse", diff)
                                    }
                                    logic::Trick::OneWall => {
                                        format!("{:?} single wall wallkicks", diff)
                                    }
                                    logic::Trick::PogoAbuse => {
                                        format!("{:?} ascendant light abuse", diff)
                                    }
                                    logic::Trick::ReverseKick => {
                                        format!("{:?} reverse wallkicks", diff)
                                    }
                                    logic::Trick::SunsetterAbuse => {
                                        format!("{:?} sunsetter backflip abuse", diff)
                                    }
                                    logic::Trick::Knowledge => {
                                        format!("{:?} knowledge", diff)
                                    }
                                });
                            }
                            logic::Lock::Location(loc) => {
                                ui.label(format!("access to: {:?}", loc));
                            }
                            logic::Lock::Movement(ability) => {
                                ui.label(format!("{:?}", ability));
                            }
                            logic::Lock::SmallKey => {
                                ui.label("small Key");
                            }
                            logic::Lock::Ending => {
                                ui.label("four big keys");
                            }
                        }
                    }
                    ui.vertical(|ui| {
                        egui::ScrollArea::both()
                            .id_source("locks")
                            .auto_shrink([false; 2])
                            .max_height(ui.available_width() / 2.0)
                            .show(ui, |ui| match &self.selected {
                                viewer::Node::Location(loc) => show(ui, &loc.locks(), 0),
                                viewer::Node::Check(check) => show(ui, &check.locks, 0),
                            })
                    });
                    ui.with_layout(
                        egui::Layout::default()
                            .with_cross_justify(true)
                            .with_cross_align(egui::Align::Center),
                        |ui| self.viewer.button(ui, "close"),
                    );
                });
                if ui
                    .button(egui::RichText::new("trick settings").size(25.0))
                    .clicked()
                {
                    self.tricks.open()
                }
                self.tricks.show(|ui| {
                    let mut combobox = |label: &str, trick: &mut logic::Difficulty| {
                        egui::ComboBox::from_label(label)
                            .selected_text(trick.as_ref())
                            .show_ui(ui, |ui| {
                                use strum::IntoEnumIterator;
                                for diff in logic::Difficulty::iter() {
                                    ui.selectable_value(trick, diff, diff.to_string());
                                }
                            });
                    };
                    combobox("momentum conservation", &mut self.momentum);
                    combobox("single wall wallkick", &mut self.one_wall);
                    combobox("reverse wallkicks", &mut self.reverse_kick);
                    combobox("sunsetter flip abuse", &mut self.sunsetter_abuse);
                    combobox("ascendant light abuse", &mut self.pogo_abuse);
                    combobox("movement", &mut self.movement);
                    combobox("cling abuse", &mut self.cling_abuse);
                    combobox("knowledge", &mut self.knowledge);
                    ui.with_layout(
                        egui::Layout::default()
                            .with_cross_justify(true)
                            .with_cross_align(egui::Align::Center),
                        |ui| self.tricks.button(ui, "close"),
                    );
                });
                if ui
                    .button(egui::RichText::new("generate and install seed").size(33.0))
                    .clicked()
                {
                    notify!(
                        self,
                        logic::randomise(self),
                        "seed has been generated, written and installed"
                    )
                }
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
                        match self.pak_str.contains("steamapps") {
                            true => std::process::Command::new(
                                #[cfg(target_os = "windows")]
                                "explorer",
                                #[cfg(target_os = "linux")]
                                "xdg-open",
                            )
                            .arg("steam://rungameid/2365810")
                            .spawn(),
                            false => std::process::Command::new(
                                self.pak
                                    .join("../../Binaries/Win64/pseudoregalia-Win64-Shipping.exe")
                            )
                            .spawn(),
                        },
                        "game found and launched successfully"
                    )
                }
            });
            self.notifs.show_dialog();
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("pak", self.pak.to_str().unwrap_or_default().to_string());
        let mut set_bool = |key: &str, value: bool| storage.set_string(key, value.to_string());
        set_bool("abilities", self.abilities);
        set_bool("aspects", self.aspects);
        set_bool("outfits", self.outfits);
        set_bool("small keys", self.small_keys);
        set_bool("big keys", self.big_keys);
        set_bool("health", self.health);
        set_bool("split greaves", self.split_greaves);
        set_bool("progressive", self.progressive);
        set_bool("goatlings", self.goatlings);
        set_bool("notes", self.notes);
        set_bool("chairs", self.chairs);
        set_bool("split cling", self.split_cling);
        set_bool("spawn", self.spawn);
        set_bool("music", self.music);
        storage.set_string("hints", self.hints.to_string());
        let mut set_difficulty =
            |key: &str, value: logic::Difficulty| storage.set_string(key, value.to_string());
        set_difficulty("momentum", self.momentum);
        set_difficulty("one wall", self.one_wall);
        set_difficulty("reverse kick", self.reverse_kick);
        set_difficulty("sunsetter abuse", self.sunsetter_abuse);
        set_difficulty("pogo abuse", self.pogo_abuse);
        set_difficulty("movement", self.movement);
        set_difficulty("cling abuse", self.cling_abuse);
        set_difficulty("knowledge", self.knowledge);
    }
}
