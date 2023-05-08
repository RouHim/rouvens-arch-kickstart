use std::fs;
use std::path::Path;

use crate::{pacman, shell, Feature};

pub struct Terminator {}

const PACKAGE_NAME: &str = "terminator";

impl Feature for Terminator {
    fn install(&self) -> bool {
        let config_dir = get_config_dir();
        let config_file = get_config_file();

        // Install terminator
        pacman::install(PACKAGE_NAME);

        // Copy config file
        shell::execute(format!("mkdir -p {config_dir}").as_str());
        let _ = fs::write(&config_file, include_bytes!("../assets/terminator-config")).is_ok();

        // Own config file for sudo user
        shell::own_file_for_user(config_file.as_str())
    }

    fn uninstall(&self) -> bool {
        // Uninstall terminator
        pacman::uninstall(PACKAGE_NAME);

        // Remove config file
        let config_dir = get_config_dir();
        shell::execute_as_root(format!("rm -rf {config_dir}"))
    }

    fn is_installed(&self) -> bool {
        let config_file = get_config_file();
        let installed = pacman::is_installed(PACKAGE_NAME);
        let config_exists = Path::new(config_file.as_str()).exists();

        installed && config_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Terminator")
    }
}

fn get_config_dir() -> String {
    shell::user_home_dir_path()
        .join(".config")
        .join("terminator")
        .to_str()
        .unwrap()
        .to_string()
}

fn get_config_file() -> String {
    shell::user_home_dir_path()
        .join(".config")
        .join("terminator")
        .join("config")
        .to_str()
        .unwrap()
        .to_string()
}
