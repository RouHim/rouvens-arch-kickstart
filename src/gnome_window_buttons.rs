use crate::Feature;

use crate::shell;
use crate::shell::RootShell;

#[derive(Clone)]
pub struct GnomeEnableWindowButtons {}

impl Feature for GnomeEnableWindowButtons {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        shell::execute("gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:minimize,maximize,close'")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        // Only close button, which is the default
        shell::execute(
            "gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:close'",
        )
    }

    fn is_installed(&self) -> bool {
        let output = shell::execute_with_output(
            "gsettings get org.gnome.desktop.wm.preferences button-layout",
        );
        output.contains("appmenu:minimize,maximize,close")
    }

    fn get_name(&self) -> String {
        String::from("Enable minimize, maximize and close buttons")
    }
}
