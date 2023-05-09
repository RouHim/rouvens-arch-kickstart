use crate::shell::RootShell;
use crate::{pacman, Feature};

pub struct ZshCompletions {}

const PACKAGE_NAME: &str = "zsh-completions";

impl Feature for ZshCompletions {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Install Zsh Completions")
    }
}
