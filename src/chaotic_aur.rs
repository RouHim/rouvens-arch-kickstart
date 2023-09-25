use scraper::Html;
use std::fs;

use crate::shell::RootShell;
use crate::Feature;

#[derive(Clone)]
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
        // Install Chaotic AUR repo
        parse_aur_install_cmds().iter().for_each(|cmd| {
            root_shell.execute(cmd);
        });

        // Add to pacman.conf
        if !pacman_config_contains_chaotic() {
            append_chaotic_to_pacman_conf(root_shell);
        }

        // Update repos
        root_shell.execute("pacman -Sc --noconfirm");

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

fn parse_aur_install_cmds() -> Vec<String> {
    let html = ureq::get("https://aur.chaotic.cx/")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let document = Html::parse_document(&html);
    let selector = scraper::Selector::parse("code.command").unwrap();
    let parsed_cmds = document.select(&selector);

    let cmds: Vec<String> = parsed_cmds
        .into_iter()
        .map(|command| {
            command
                .inner_html()
                .replace('\n', "")
                .as_str()
                .trim()
                .to_string()
        })
        .map(|command| {
            if command.starts_with("pacman -U") {
                format!("{} --noconfirm", command)
            } else {
                command
            }
        })
        .collect();
    cmds
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
