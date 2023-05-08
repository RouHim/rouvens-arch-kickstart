use crate::{pacman, shell, Feature};

pub struct GnomeShellExtensionSystemMonitor {}

impl Feature for GnomeShellExtensionSystemMonitor {
    fn install(&self) -> bool {
        pacman::install("libgtop networkmanager gnome-system-monitor clutter");
        shell::execute("gnome-extensions enable system-monitor@paradoxxx.zero.gmail.com")
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall("gnome-system-monitor")
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed("gnome-system-monitor")
    }

    fn get_name(&self) -> String {
        String::from("Gnome System Monitor extension")
    }
}
