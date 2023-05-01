use std::fs;
use std::path::Path;

use crate::{pacman, shell, Feature};

pub struct FootTerminal {}

const PACKAGE_NAME: &str = "foot";

impl Feature for FootTerminal {
    fn install(&self) -> bool {
        let config_dir = get_config_dir();
        let config_file = get_config_file();

        // Install foot terminal
        pacman::install_needed(PACKAGE_NAME);

        // Copy config file
        shell::execute_as_user(format!("mkdir -p {config_dir}").as_str());
        let _ = fs::write(&config_file, include_bytes!("../assets/foot-config")).is_ok();

        // Own config file for sudo user
        shell::own_file_for_sudo_user(config_file.as_str())
    }

    fn uninstall(&self) -> bool {
        // Uninstall foot terminal
        pacman::uninstall(PACKAGE_NAME);

        // Remove config file
        let config_dir = get_config_dir();
        shell::execute(format!("rm -rf {config_dir}"))
    }

    fn is_installed(&self) -> bool {
        let config_file = get_config_file();
        let config_exists = Path::new(config_file.as_str()).exists();
        let installed = pacman::is_installed(PACKAGE_NAME);
        installed && config_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Foot Terminal")
    }
}

/// Get the config directory for foot terminal
/// Default location is ~/.config/foot
fn get_config_dir() -> String {
    shell::sudo_user_home_dir()
        .join(".config")
        .join("foot")
        .to_str()
        .unwrap()
        .to_string()
}

/// Get the config file for foot terminal
/// Default location is ~/.config/foot/foot.ini
fn get_config_file() -> String {
    shell::sudo_user_home_dir()
        .join(".config")
        .join("foot")
        .join("foot.ini")
        .to_str()
        .unwrap()
        .to_string()
}
