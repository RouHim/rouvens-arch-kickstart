use crate::chaotic_aur::ChaoticAur;
use crate::{pacman, shell, Feature};

pub struct GnomeShellExtensionDashToPanel {}

const PACKAGE_NAME: &str = "gnome-shell-extension-dash-to-panel";

impl Feature for GnomeShellExtensionDashToPanel {
    fn install(&self) -> bool {
        ChaoticAur {}.install();
        pacman::install(PACKAGE_NAME);
        shell::execute("gnome-extensions enable dash-to-panel@jderose9.github.com")
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Dash To Panel")
    }
}
