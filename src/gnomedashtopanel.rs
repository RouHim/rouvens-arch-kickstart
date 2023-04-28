use crate::{pacman, Feature};

pub struct DashToPanel {}

const PACKAGE_NAME: &str = "gnome-shell-extension-dash-to-panel";

impl Feature for DashToPanel {
    fn install(&self) -> bool {
        pacman::install(PACKAGE_NAME)
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
