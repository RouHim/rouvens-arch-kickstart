use crate::{pacman, zshrc, Feature};

pub struct ZshCompletions {}

const PACKAGE_NAME: &str = "zsh-completions";

impl Feature for ZshCompletions {
    fn install(&self) -> bool {
        // Install package using pacman
        let pacman_is_ok = pacman::install(PACKAGE_NAME);

        pacman_is_ok
    }

    fn uninstall(&self) -> bool {
        // Uninstall using pacman
        let pacman_is_ok = pacman::uninstall(PACKAGE_NAME);

        pacman_is_ok
    }

    fn is_installed(&self) -> bool {
        // Check if package is installed
        let is_installed = pacman::is_installed(PACKAGE_NAME);

        is_installed
    }

    fn get_name(&self) -> String {
        String::from("Install Zsh Completions")
    }
}
