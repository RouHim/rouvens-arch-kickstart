use std::fs;

use std::path::PathBuf;

use crate::shell::RootShell;

use crate::Feature;

#[derive(Clone)]
pub struct CommonSystemFixes {}

impl Feature for CommonSystemFixes {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Set MOZ_ENABLE_WAYLAND=1 in /etc/environment
        // using root shell
        root_shell.execute("echo 'MOZ_ENABLE_WAYLAND=1' >> /etc/environment");


        CommonSystemFixes::is_installed(self)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // remove the MOZ_ENABLE_WAYLAND=1 from /etc/environment
        // using root shell
        root_shell.execute("sed -i '/MOZ_ENABLE_WAYLAND=1/d' /etc/environment");

        !CommonSystemFixes::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        // Check if MOZ_ENABLE_WAYLAND=1 is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(environment_file).unwrap();
        environment_file_contents.contains("MOZ_ENABLE_WAYLAND=1")
    }

    fn get_name(&self) -> String {
        String::from("Apply common system fixes")
    }
}
