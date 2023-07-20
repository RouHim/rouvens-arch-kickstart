use crate::Feature;

use crate::shell;
use crate::shell::RootShell;

#[derive(Clone)]
pub struct GnomeOverAmplification {}

impl Feature for GnomeOverAmplification {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        // Enable gnome over amplification
        shell::execute("gsettings set org.gnome.desktop.sound allow-volume-above-100-percent true")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        // Disable gnome over amplification
        shell::execute("gsettings set org.gnome.desktop.sound allow-volume-above-100-percent false")
    }

    fn is_installed(&self) -> bool {
        // Check if gnome over amplification is enabled
        shell::execute_with_output(
            "gsettings get org.gnome.desktop.sound allow-volume-above-100-percent",
        )
        .contains("true")
    }

    fn get_name(&self) -> String {
        String::from("Enable gnome over amplification")
    }
}
