use crate::chaotic_aur::ChaoticAur;
use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

pub struct GnomeShellExtensionDashToPanel {}

const PACKAGE_NAME: &str = "gnome-shell-extension-dash-to-panel";

impl Feature for GnomeShellExtensionDashToPanel {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        ChaoticAur {}.install(root_shell);
        pacman::install(PACKAGE_NAME, root_shell);
        shell::execute("gnome-extensions enable dash-to-panel@jderose9.github.com")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Dash To Panel")
    }
}
