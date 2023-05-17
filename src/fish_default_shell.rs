use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
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

"#;

impl Feature for FishDefaultShell {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install("fish fisher", root_shell);

        append_to_fish_config(FISH_CONFIG);

        let username = shell::get_current_user();
        shell::execute(format!("pkexec chsh --shell $(which fish) {username}"));

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
        let is_default_shell = shell::execute(format!("cat /etc/passwd | grep {username} | grep /fish"));

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

fn append_to_fish_config(line_to_append: &str) -> bool {
    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_fish_config().as_path())
    {
        writeln!(file, "{}", line_to_append).expect("Failed to write to file");
        true
    } else {
        false
    }
}

pub fn remove_from_fish_config(line_to_remove: &str) -> bool {
    if let Ok(file) = OpenOptions::new()
        .read(true)
        .open(get_fish_config().as_path())
    {
        let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(get_fish_config().as_path())
            .unwrap();

        for line in lines {
            if line.trim() != line_to_remove {
                writeln!(file, "{}", line).expect("Failed to write to file");
            }
        }

        true
    } else {
        false
    }
}
