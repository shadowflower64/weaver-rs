use log::info;

use crate::file_dialog::{CompatFileDialog, HtmlInputElement};
use std::path::PathBuf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WeaverApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    file_dialog: CompatFileDialog,
    #[serde(skip)]
    picked_file: Option<PathBuf>,
}

impl Default for WeaverApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            file_dialog: CompatFileDialog::placeholder(),
            picked_file: None,
        }
    }
}

impl WeaverApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, html_file_input: Option<HtmlInputElement>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let file_dialog = CompatFileDialog::new(html_file_input);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            WeaverApp {
                file_dialog,
                ..eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
            }
        } else {
            WeaverApp {
                file_dialog,
                ..Default::default()
            }
        }
    }
}

impl eframe::App for WeaverApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Update the dialog
        self.file_dialog.update(ui);

        // Check if the user picked a file.
        if let Some(path) = self.file_dialog.take_picked() {
            info!("file picked now");
            self.picked_file = Some(path.to_path_buf());
        }

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.file_dialog.pick_file();
                    }

                    // NOTE: no File->Quit on web pages!
                    #[expect(clippy::collapsible_if)]
                    if !is_web {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                    if ui.button("Quit 2").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    #[allow(clippy::allow_attributes)]
                    #[allow(unused)]
                    let random_string = "<random not available>".to_owned();

                    #[cfg(not(target_arch = "wasm32"))]
                    let random_string = {
                        use rand::{RngExt as _, distr::Uniform, rng};
                        use std::char;
                        rng()
                            .sample_iter(Uniform::new_inclusive(0, 9).expect("invalid range"))
                            .take(5)
                            .filter_map(|i| char::from_digit(i, 10))
                            .collect::<String>()
                    };

                    if ui.button(format!("Immediate mode - random string: {random_string}")).clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Weaver Chart Editor");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.add_space(16.0);
            ui.label(format!("Picked file: {:?}", self.picked_file));

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "eframe Template Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
        ui.label(".");
    });
}
