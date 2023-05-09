use crate::shell;
use crate::shell::RootShell;
use crate::Feature;

pub struct GnomeDarkMode {}

impl Feature for GnomeDarkMode {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        shell::execute("gsettings set org.gnome.desktop.interface color-scheme 'prefer-dark'")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        shell::execute("gsettings set org.gnome.desktop.interface color-scheme 'prefer-light'")
    }

    fn is_installed(&self) -> bool {
        shell::execute_with_output("gsettings get org.gnome.desktop.interface color-scheme")
            .contains("prefer-dark")
    }

    fn get_name(&self) -> String {
        String::from("Enable Dark mode")
    }
}