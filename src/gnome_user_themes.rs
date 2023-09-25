use crate::shell::RootShell;
use crate::{shell, Feature};

#[derive(Clone)]
pub struct GnomeShellExtensionUserThemes {}

const EXTENSION_URL: &str = "https://extensions.gnome.org/extension/19/user-themes/";
const EXTENSION_ID: &str = "user-theme@gnome-shell-extensions.gcampax.github.com";

impl Feature for GnomeShellExtensionUserThemes {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        shell::execute(format!("xdg-open {EXTENSION_URL}"))
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        shell::execute(format!("gnome-extensions uninstall {EXTENSION_ID}"))
    }

    fn is_installed(&self) -> bool {
        shell::execute(format!("gnome-extensions info {EXTENSION_ID}"))
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell User Themes")
    }
}
