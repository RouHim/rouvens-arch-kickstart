use std::path::Path;

use crate::{pacmanconfig, shell, Feature};

pub struct ChaoticAur {}

impl Feature for ChaoticAur {
    fn install(&self) -> bool {
        shell::execute("pacman-key --init");
        shell::execute("pacman-key --recv-key FBA220DFC880C036 --keyserver keyserver.ubuntu.com");
        shell::execute("pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst' --noconfirm");
        pacmanconfig::add_to_pacman_conf(
            "[chaotic-aur]\nInclude = /etc/pacman.d/chaotic-mirrorlist",
        )
    }

    fn uninstall(&self) -> bool {
        pacmanconfig::remove_from_pacman_conf();
        let response = shell::execute("pacman -Rns chaotic-keyring chaotic-mirrorlist");
        shell::execute("rm -rf /etc/pacman.d/chaotic-mirrorlist");
        shell::execute("pacman -Sc --noconfirm");
        response
    }

    fn is_installed(&self) -> bool {
        is_active()
    }

    fn get_name(&self) -> String {
        String::from("Install Chaotic AUR")
    }
}

pub fn is_active() -> bool {
    let pacman_conf = pacmanconfig::read_pacman_conf();

    let has_chaotic_aur_section = pacman_conf
        .iter()
        .any(|line| line.trim() == "[chaotic-aur]");

    let has_chaotic_mirrorlist = Path::new("/etc/pacman.d/chaotic-mirrorlist").exists();

    has_chaotic_aur_section && has_chaotic_mirrorlist
}
