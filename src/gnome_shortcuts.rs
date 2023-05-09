use crate::shell;
use crate::shell::RootShell;
use crate::Feature;
use std::fs;

pub struct GnomeKeyboardShortcuts {}

impl Feature for GnomeKeyboardShortcuts {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        let temp_file = "/tmp/gnome_shortcuts.sh";

        // Cleanup existing config file
        root_shell.execute(format!("rm -rf {temp_file}"));

        // Copy config file
        let _ = fs::write(temp_file, include_bytes!("../assets/gnome_shortcuts.sh")).is_ok();

        // Own config file for sudo user and make executable
        shell::execute(format!("chmod +x {temp_file}").as_mut_str());

        // Execute it as user

        // Delete temp file
        //shell::execute(format!("rm -rf {temp_file}"));

        shell::execute(temp_file)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        true
    }

    fn is_installed(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("Gnome keyboard shortcuts")
    }
}
