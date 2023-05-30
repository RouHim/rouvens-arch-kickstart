use crate::shell::RootShell;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct PacmanPamac {}

const PACKAGE_NAME: &str = "pamac";

impl Feature for PacmanPamac {
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
        String::from("Install Pamac")
    }
}
