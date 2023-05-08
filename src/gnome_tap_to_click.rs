use crate::shell;
use crate::Feature;

pub struct GnomeTapToClick {}

impl Feature for GnomeTapToClick {
    fn install(&self) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click true")
    }

    fn uninstall(&self) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click false")
    }

    fn is_installed(&self) -> bool {
        let output = shell::execute_with_output(
            "gsettings get org.gnome.desktop.peripherals.touchpad tap-to-click",
        );
        output.contains("true")
    }

    fn get_name(&self) -> String {
        String::from("Enable Tap to Click")
    }
}
