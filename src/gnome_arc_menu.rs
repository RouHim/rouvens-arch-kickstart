use crate::{yay, Feature};

pub struct GnomeShellExtensionArcMenu {}

const PACKAGE_NAME: &str = "gnome-shell-extension-arc-menu";

impl Feature for GnomeShellExtensionArcMenu {
    fn install(&self) -> bool {
        // das funktioniert nicht, weil yay als user ausgeführt werden muss
        yay::install(PACKAGE_NAME)
    }

    fn uninstall(&self) -> bool {
        yay::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        yay::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Arc Menu")
    }
}
