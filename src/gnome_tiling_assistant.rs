use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

#[derive(Clone)]
pub struct GnomeShellExtensionTilingAssistant {}

const PACKAGE_NAME: &str = "gnome-shell-extension-tiling-assistant-git";

impl Feature for GnomeShellExtensionTilingAssistant {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell);
        shell::execute("gnome-extensions enable tiling-assistant@leleat-on-github")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Tiling Assistant")
    }
}
