use crate::shell::RootShell;
use crate::{pacman, shell, Feature, yay};

pub struct GnomeShellExtensionArcMenu {}

const PACKAGE_NAME: &'static str = "gnome-shell-extension-arc-menu";

impl Feature for GnomeShellExtensionArcMenu {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        yay::install(PACKAGE_NAME)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        yay::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        yay::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome System Monitor extension")
    }
}
