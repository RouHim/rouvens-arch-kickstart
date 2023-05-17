use crate::shell::RootShell;
use crate::{pacman, shell, Feature};
use eframe::egui::TextBuffer;

pub struct GnomeShellExtensionBlurMyShell {}

const PACKAGE_NAME: &str = "gnome-shell-extension-blur-my-shell";

impl Feature for GnomeShellExtensionBlurMyShell {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell);
        shell::execute("gnome-extensions enable blur-my-shell@aunetx".as_str())
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Blur my Shell")
    }
}
