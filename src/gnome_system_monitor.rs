use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

pub struct GnomeShellExtensionSystemMonitor {}

impl Feature for GnomeShellExtensionSystemMonitor {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(
            "libgtop networkmanager gnome-system-monitor clutter",
            root_shell,
        );
        shell::execute("gnome-extensions enable system-monitor@paradoxxx.zero.gmail.com")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall("gnome-system-monitor", root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed("gnome-system-monitor")
    }

    fn get_name(&self) -> String {
        String::from("Gnome System Monitor extension")
    }
}
