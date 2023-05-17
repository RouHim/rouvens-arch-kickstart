use crate::shell::RootShell;

use crate::{pacman, shell, Feature};

pub struct Rust {}

const PACKAGE_NAME: &str = "rustup";

impl Feature for Rust {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell);
        shell::execute("rustup default stable")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Install Rust")
    }
}
