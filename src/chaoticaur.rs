use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{pacmanconfig, shell, Feature};

const CHAOTIC_AUR_README: &str =
    "https://raw.githubusercontent.com/chaotic-aur/.github/main/profile/README.md";

lazy_static! {
    static ref PACMAN_COMMAND_PATTERN: Regex = Regex::new(r"\-\s*`#\s(?P<cmds>pacman.*)`").unwrap();
}

pub struct ChaoticAur {}

impl Feature for ChaoticAur {
    fn install(&self) -> bool {
        shell::execute("pacman-key --init");
        shell::execute(
            "pacman-key --recv-key FBA220DFC880C036 --keyserver keyserver.ubuntu.com",
        );
        shell::execute("pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst' --noconfirm");
        pacmanconfig::add_to_pacman_conf(
            "[chaotic-aur]\nInclude = /etc/pacman.d/chaotic-mirrorlist",
        )
    }

    fn uninstall(&self) -> bool {
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

pub fn get_install_commands() -> Option<Vec<String>> {
    download_readme().map(extract_commands)
}

fn extract_commands(readme_content: String) -> Vec<String> {
    let mut cmds: Vec<String> = vec![];
    for capture in PACMAN_COMMAND_PATTERN.captures_iter(readme_content.as_str()) {
        cmds.push(capture.name("cmds").unwrap().as_str().to_string());
    }
    cmds
}

fn download_readme() -> Option<String> {
    ureq::get(CHAOTIC_AUR_README)
        .call()
        .ok()
        .and_then(|response| response.into_string().ok())
}

pub fn is_active() -> bool {
    let pacman_conf = pacmanconfig::read_pacman_conf();

    let has_chaotic_aur_section = pacman_conf
        .iter()
        .any(|line| line.trim() == "[chaotic-aur]");

    let has_chaotic_mirrorlist = Path::new("/etc/pacman.d/chaotic-mirrorlist").exists();

    has_chaotic_aur_section && has_chaotic_mirrorlist
}