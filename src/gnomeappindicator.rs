use std::process::Command;

use crate::{chaoticaur, Feature, pacman, yay};

pub struct GnomeShellExtensionAppIndicator {}

const PACKAGE_NAME: &str = "gnome-shell-extension-appindicator";

impl Feature for GnomeShellExtensionAppIndicator {
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
        String::from("Gnome Shell Extension App Indicator")
    }
}
