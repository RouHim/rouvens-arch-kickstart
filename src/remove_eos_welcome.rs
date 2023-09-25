use crate::shell::RootShell;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct RemoveEosWelcome {}

const PACKAGE_NAME: &str = "welcome";

impl Feature for RemoveEosWelcome {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Remove the default welcome package
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Re-Install the default welcome package
        pacman::install(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        !pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Remove EOS welcome")
    }
}
