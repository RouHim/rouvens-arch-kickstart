use crate::shell::RootShell;
use crate::{shell, yay, Feature};

#[derive(Clone)]
pub struct GnomeShellExtensionJustPerfection {}

const PACKAGE_NAME: &str = "gnome-shell-extension-just-perfection-desktop";

impl Feature for GnomeShellExtensionJustPerfection {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        yay::install(PACKAGE_NAME);
        shell::execute("gnome-extensions enable just-perfection-desktop@just-perfection")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        yay::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        yay::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension Just Perfection")
    }
}
