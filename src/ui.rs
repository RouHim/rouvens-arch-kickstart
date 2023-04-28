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
            ui.heading(TITLE);
            ui.separator();

            if ui.button("Check feature states").clicked() {
                for feature in &mut self.features {
                    self.feature_state
                        .insert(feature.get_name(), feature.is_installed());
                }
            };

            ui.separator();

            for feature in &mut self.features {
                let name = feature.get_name();

                let button_text = if *self.feature_state.get(&name).unwrap() {
                    "Uninstall"
                } else {
                    "Install"
                };

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(&name);

                        ui.separator();

                        let install_button = ui.button(button_text);
                        if install_button.clicked() {
                            if *self.feature_state.get(&name).unwrap() {
                                feature.uninstall();
                            } else {
                                feature.install();
                            }
                            self.feature_state
                                .insert(feature.get_name(), feature.is_installed());
                        };
                    });
                });
            }
        });
    }
}

impl AppState {}
