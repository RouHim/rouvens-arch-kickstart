use crate::Feature;

use crate::shell;
use crate::shell::RootShell;

pub struct GnomeDisableMouseAcceleration {}

impl Feature for GnomeDisableMouseAcceleration {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.mouse accel-profile 'flat'")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.mouse accel-profile 'default'")
    }

    fn is_installed(&self) -> bool {
        let output = shell::execute_with_output(
            "gsettings get org.gnome.desktop.peripherals.mouse accel-profile",
        );
        output.contains("flat")
    }

    fn get_name(&self) -> String {
        String::from("Disable mouse acceleration")
    }
}
