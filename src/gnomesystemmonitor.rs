use crate::{pacman, Feature};

pub struct GnomeShellSystemMonitor {}

impl Feature for GnomeShellSystemMonitor {
    fn install(&self) -> bool {
        pacman::install("libgtop networkmanager gnome-system-monitor clutter")
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
