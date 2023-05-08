use crate::{pacman, shell, Feature};
use eframe::egui::TextBuffer;

pub struct GnomeShellExtensionAppIndicator {}

const PACKAGE_NAME: &str = "gnome-shell-extension-appindicator";

impl Feature for GnomeShellExtensionAppIndicator {
    fn install(&self) -> bool {
        pacman::install(PACKAGE_NAME);
        shell::execute("gnome-extensions enable appindicatorsupport@rgcjonas.gmail.com".as_str())
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension App Indicator")
    }
}
