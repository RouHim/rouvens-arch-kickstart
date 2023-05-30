use crate::shell::RootShell;
use crate::{pacman, shell, yay, Feature};

#[derive(Clone)]
pub struct GnomeShellExtensionSystemMonitor {}

const PACKAGE_NAME: &str = "gnome-shell-extension-system-monitor-next-git";

impl Feature for GnomeShellExtensionSystemMonitor {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(
            "libgtop networkmanager gnome-system-monitor clutter",
            root_shell,
        );
        yay::install(PACKAGE_NAME);
        shell::execute("gnome-extensions enable system-monitor@paradoxxx.zero.gmail.com")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        yay::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        yay::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Gnome Shell Extension System Monitor Next")
    }
}
