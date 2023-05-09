use crate::shell::RootShell;
use crate::{pacman, shell, Feature};
use eframe::egui::TextBuffer;

pub struct GnomeShellExtensionAppIndicator {}

const PACKAGE_NAME: &str = "gnome-shell-extension-appindicator";

impl Feature for GnomeShellExtensionAppIndicator {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell);
        shell::execute("gnome-extensions enable appindicatorsupport@rgcjonas.gmail.com".as_str())
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension App Indicator")
    }
}
