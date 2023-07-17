use std::fs;

use std::path::PathBuf;

use crate::shell::RootShell;

use crate::Feature;

#[derive(Clone)]
pub struct CommonSystemFixes {}

impl Feature for CommonSystemFixes {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        // Set MOZ_ENABLE_WAYLAND=1 in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        if !environment_file_contents.contains("MOZ_ENABLE_WAYLAND=1") {
            fs::write(&environment_file, "MOZ_ENABLE_WAYLAND=1").unwrap();
        }

        CommonSystemFixes::is_installed(self)
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        // remove the MOZ_ENABLE_WAYLAND=1 from /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        if environment_file_contents.contains("MOZ_ENABLE_WAYLAND=1") {
            fs::write(&environment_file, "MOZ_ENABLE_WAYLAND=").unwrap();
        }

        !CommonSystemFixes::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        // Check if MOZ_ENABLE_WAYLAND=1 is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        let is_moz_enable_wayland_set = environment_file_contents.contains("MOZ_ENABLE_WAYLAND=1");

        is_moz_enable_wayland_set
    }

    fn get_name(&self) -> String {
        String::from("Apply common system fixes")
    }
}
