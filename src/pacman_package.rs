use crate::shell::RootShell;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct PacmanPackage {
    pub package_name: &'static str,
    pub description: &'static str,
}

impl Feature for PacmanPackage {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(self.package_name, root_shell)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(self.package_name, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(self.package_name)
    }

    fn get_name(&self) -> String {
        self.description.to_string()
    }
}
