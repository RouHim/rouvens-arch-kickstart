use crate::shell::RootShell;
use crate::{pacman, shell, Feature};
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct PacmanImprovements {}

const PACCACHE_SERVICE: &str = r#"
[Unit]
Description=Clean-up old pacman pkg cache

[Timer]
OnCalendar=monthly
Persistent=true

[Install]
WantedBy=multi-user.target
"#;

impl Feature for PacmanImprovements {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Enable color in pacman config using root shell
        root_shell.execute("sed -i 's/#Color/Color/' /etc/pacman.conf");

        // Install pacman-contrib
        pacman::install("pacman-contrib", root_shell);
        root_shell.execute(format!(
            "echo '{PACCACHE_SERVICE}' > /etc/systemd/system/paccache.timer"
        ));
        root_shell.execute("systemctl enable paccache.timer");
        root_shell.execute("systemctl start paccache.timer");

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Disable color in pacman config using root shell
        root_shell.execute("sed -i 's/Color/#Color/' /etc/pacman.conf");

        // Uninstall pacman-contrib
        pacman::uninstall("pacman-contrib", root_shell);
        root_shell.execute("rm -rf /etc/systemd/system/paccache.timer");

        true
    }

    fn is_installed(&self) -> bool {
        //  Check if color is enabled in pacman config
        let pacman_config_file = PathBuf::from("/etc/pacman.conf");
        let pacman_config_file_contents = fs::read_to_string(pacman_config_file).unwrap();
        let is_color_enabled = pacman_config_file_contents.contains("Color");

        // Check if pacman-contrib is installed
        let is_pacman_contrib_installed = pacman::is_installed("pacman-contrib");

        // Check if service is enabled
        let is_paccache_service_enabled = shell::execute("systemctl status paccache.timer");

        is_color_enabled && is_pacman_contrib_installed && is_paccache_service_enabled
    }

    fn get_name(&self) -> String {
        String::from("Setup PacmanConfig")
    }
}
