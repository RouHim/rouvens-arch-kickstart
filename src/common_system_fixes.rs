use std::fs;

use std::path::PathBuf;

use crate::shell::RootShell;

use crate::Feature;

#[derive(Clone)]
pub struct CommonSystemFixes {}

impl Feature for CommonSystemFixes {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Set MOZ_ENABLE_WAYLAND=1 in /etc/environment
        if !root_shell.execute("grep -q MOZ_ENABLE_WAYLAND=1 /etc/environment") {
            root_shell.execute("echo 'MOZ_ENABLE_WAYLAND=1' >> /etc/environment");
        }

        // Limit journald size, create a new file in /etc/systemd/journald.conf.d/
        Self::limit_journald_log_size(root_shell);

        CommonSystemFixes::is_installed(self)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // remove the MOZ_ENABLE_WAYLAND=1 from /etc/environment
        // using root shell
        root_shell.execute("sed -i '/MOZ_ENABLE_WAYLAND=1/d' /etc/environment");

        // Remove the journald limit file
        root_shell.execute("rm -rf /etc/systemd/journald.conf.d/limit.conf");

        !CommonSystemFixes::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        // Check if MOZ_ENABLE_WAYLAND=1 is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(environment_file).unwrap();

        // Check if limit.conf exists
        let limit_file_exists = PathBuf::from("/etc/systemd/journald.conf.d/limit.conf").exists();

        limit_file_exists && environment_file_contents.contains("MOZ_ENABLE_WAYLAND=1")
    }

    fn get_name(&self) -> String {
        String::from("Apply common system fixes")
    }
}

impl CommonSystemFixes {
    fn limit_journald_log_size(root_shell: &mut RootShell) {
        // Remove file if exists
        root_shell.execute("rm -rf /etc/systemd/journald.conf.d/limit.conf");

        root_shell.execute("mkdir -p /etc/systemd/journald.conf.d/");
        root_shell.execute("echo '[Journal]' > /etc/systemd/journald.conf.d/limit.conf");
        root_shell.execute("echo 'SystemMaxUse=50M' >> /etc/systemd/journald.conf.d/limit.conf");
        root_shell
            .execute("echo 'SystemMaxFileSize=10M' >> /etc/systemd/journald.conf.d/limit.conf");
        root_shell.execute("echo 'SystemKeepFree=100M' >> /etc/systemd/journald.conf.d/limit.conf");
        root_shell.execute("echo 'SystemMaxFiles=5' >> /etc/systemd/journald.conf.d/limit.conf");
    }
}
