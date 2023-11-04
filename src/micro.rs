use std::fs;

use crate::shell::RootShell;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct Micro {}

const PACKAGE_NAME: &str = "micro";

impl Feature for Micro {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Install package
        pacman::install(PACKAGE_NAME, root_shell);

        // Set micro as default EDITOR in /etc/environment
        if !fs::read_to_string("/etc/environment")
            .unwrap()
            .contains("EDITOR=micro")
        {
            root_shell.execute("echo 'EDITOR=micro' >> /etc/environment");
        };

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Uninstall package
        pacman::uninstall(PACKAGE_NAME, root_shell);

        // Remove micro as default EDITOR in /etc/environment
        root_shell.execute("sed -i '/EDITOR=micro/d' /etc/environment")
    }

    fn is_installed(&self) -> bool {
        // Check if micro is installed
        let package_installed = pacman::is_installed(PACKAGE_NAME);

        // Check if micro is set as default EDITOR in /etc/environment
        let environment_file_contents = fs::read_to_string("/etc/environment").unwrap();
        let editor_set = environment_file_contents.contains("EDITOR=micro");

        package_installed && editor_set
    }

    fn get_name(&self) -> String {
        String::from("Setup Micro")
    }
}
