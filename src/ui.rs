use std::collections::HashMap;

use eframe::egui;

use crate::Feature;

const TITLE: &str = "Rouvens Arch Kickstart";

pub fn show(features: Vec<Box<dyn Feature>>) -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 720.0)),
        ..Default::default()
    };

    let mut feature_state = HashMap::<String, bool>::new();

    for feature in &features {
        let name = feature.get_name();
        let is_installed = feature.is_installed();
        feature_state.insert(name, is_installed);
    }

    let initial_state = Box::<AppState>::new(AppState {
        features,
        feature_state,
    });

    eframe::run_native(TITLE, options, Box::new(|_cc| initial_state))
}

struct AppState {
    features: Vec<Box<dyn Feature>>,
    feature_state: HashMap<String, bool>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(TITLE);
                if ui.button("Check feature states").clicked() {
                    for feature in &mut self.features {
                        self.feature_state
                            .insert(feature.get_name(), feature.is_installed());
                    }
                };
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for feature in &mut self.features {
                    let name = feature.get_name();

                    if feature.is_group_element() {
                        ui.add_space(20.0);
                        ui.heading(&name);
                        ui.separator();
                        continue;
                    }

                    let is_installed = *self.feature_state.get(&name).unwrap();

                    ui.horizontal(|ui| {
                        ui.monospace(format!("{}:", name));

                        ui.separator();

                        let install_button =
                            ui.button(if is_installed { "Uninstall" } else { "Install" });
                        if install_button.clicked() {
                            // Install or uninstall
                            let ok = if is_installed {
                                feature.uninstall()
                            } else {
                                feature.install()
                            };

                            if !ok {
                                ui.add(egui::Label::new("Failed to install/uninstall"));
                            }

                            // Update state
                            self.feature_state
                                .insert(feature.get_name(), feature.is_installed());
                        };
                    });
                }
            });
        });
    }
}

impl AppState {}
