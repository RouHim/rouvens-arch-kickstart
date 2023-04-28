use crate::{pacman, Feature};

pub struct Pamac {}

const PACKAGE_NAME: &str = "pamac-aur";

impl Feature for Pamac {
    fn install(&self) -> bool {
        pacman::install(PACKAGE_NAME)
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall(PACKAGE_NAME)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Install Pamac")
    }
}
