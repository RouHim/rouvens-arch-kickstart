use std::fs;

use crate::shell::RootShell;
use crate::{shell, Feature};

pub struct ChaoticAur {}

const PACMAN_CONFIG_FILE: &str = "/etc/pacman.conf";
const CHAOTIC_AUR_SECTION_HEAD: &str = "[chaotic-aur]";
const CHAOTIC_AUR_SECTION_DATA: &str = "Include = /etc/pacman.d/chaotic-mirrorlist";
const CHAOTIC_AUR_SECTION: &str = r#"

[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist
"#;

impl Feature for ChaoticAur {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        shell::execute("pacman-key --init");
        root_shell
            .execute("pacman-key --recv-key FBA220DFC880C036 --keyserver keyserver.ubuntu.com");
        root_shell.execute("pacman-key --lsign-key FBA220DFC880C036");
        root_shell.execute("pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst' --noconfirm");

        if !pacman_config_contains_chaotic() {
            append_chaotic_to_pacman_conf(root_shell);
        }

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        remove_chaotic_from_pacman_conf();
        let response = root_shell.execute("pacman -Rns chaotic-keyring chaotic-mirrorlist");
        root_shell.execute("rm -rf /etc/pacman.d/chaotic-mirrorlist");
        root_shell.execute("pacman -Sc --noconfirm");
        response
    }

    fn is_installed(&self) -> bool {
        pacman_config_contains_chaotic()
    }

    fn get_name(&self) -> String {
        String::from("Install Chaotic AUR")
    }
}

fn remove_chaotic_from_pacman_conf() {
    let pacman_config = fs::read(PACMAN_CONFIG_FILE).unwrap();
    let mut pacman_config = String::from_utf8(pacman_config).unwrap();
    pacman_config = pacman_config.replace(CHAOTIC_AUR_SECTION_HEAD, "");
    pacman_config = pacman_config.replace(CHAOTIC_AUR_SECTION_DATA, "");
    pacman_config = pacman_config.trim().to_string();
    fs::write(PACMAN_CONFIG_FILE, pacman_config).unwrap();
}

fn append_chaotic_to_pacman_conf(root_shell: &mut RootShell) {
    root_shell.execute(format!(
        "echo '{}' >> {}",
        CHAOTIC_AUR_SECTION, PACMAN_CONFIG_FILE
    ));
}

fn pacman_config_contains_chaotic() -> bool {
    let pacman_conf = fs::read_to_string(PACMAN_CONFIG_FILE).unwrap();
    pacman_conf.contains(CHAOTIC_AUR_SECTION_HEAD) && pacman_conf.contains(CHAOTIC_AUR_SECTION_DATA)
}
