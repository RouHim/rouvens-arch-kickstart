use std::fs;

use std::path::PathBuf;

use crate::shell::RootShell;
use crate::yay::is_installed;
use crate::{pacman, shell, Feature};

pub struct FishDefaultShell {}

const FISH_CONFIG: &str = r#"

# Greeting text
set fish_greeting

# Expand path
set -x PATH /home/rouven/.local/bin $PATH

# Alias-Definitionen
alias ll 'ls -l'
alias l 'ls -la'

# Fix ssh issues
alias ssh="kitty +kitten ssh"

"#;

impl Feature for FishDefaultShell {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install("fish fisher ttf-meslo-nerd", root_shell);

        // Ensure fish config folder and file exists
        let fish_config = get_fish_config();
        if let Some(parent) = fish_config.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(fish_config.as_path(), "").unwrap();

        append_to_fish_config(FISH_CONFIG);

        // Install tide via fisher
        shell::execute("fish -c 'fisher install IlanCosman/tide'");

        let username = shell::get_current_user();
        shell::execute(format!("pkexec chsh --shell $(which fish) {username}"));

        // Open default terminal and execute: tide configure
        shell::execute("kgx --command \"fish -c 'tide configure'\"");

        is_installed("fish fisher")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        remove_from_fish_config(FISH_CONFIG);
        pacman::uninstall("fish fisher", _root_shell);
        let username = shell::get_current_user();
        shell::execute(format!("pkexec chsh --shell $(which bash) {username}"))
    }

    fn is_installed(&self) -> bool {
        let username = shell::get_current_user();
        let is_default_shell =
            shell::execute(format!("cat /etc/passwd | grep {username} | grep /fish"));

        let is_installed = pacman::is_installed("fish");

        is_installed && is_default_shell
    }

    fn get_name(&self) -> String {
        String::from("Set FISH as default shell")
    }
}

fn get_fish_config() -> PathBuf {
    shell::user_home_dir_path()
        .join(".config")
        .join("fish")
        .join("config.fish")
}

fn append_to_fish_config(string_to_append: &str) -> bool {
    let config_file = get_fish_config();
    let content = fs::read_to_string(config_file.as_path()).unwrap();
    let new_content = format!("{}\n{}", content, string_to_append);
    fs::write(config_file.as_path(), new_content).is_ok()
}

pub fn remove_from_fish_config(string_to_remove: &str) -> bool {
    let config_file = get_fish_config();
    fs::read_to_string(config_file.as_path())
        .map(|content| {
            let new_content = content.replace(string_to_remove, "");
            fs::write(config_file.as_path(), new_content)
        })
        .is_ok()
}
