use crate::{pacman, Feature};

pub struct PacmanPackage {
    pub package_name: &'static str,
}

impl Feature for PacmanPackage {
    fn install(&self) -> bool {
        pacman::install(self.package_name)
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall(self.package_name)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(self.package_name)
    }

    fn get_name(&self) -> String {
        format!("Install {}", self.package_name)
    }
}
