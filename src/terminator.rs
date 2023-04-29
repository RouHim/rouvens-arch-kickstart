use std::fs;
use std::path::Path;

use crate::{Feature, pacman, shell};

pub struct Terminator {}

const PACKAGE_NAME: &str = "terminator";

impl Feature for Terminator {
    fn install(&self) -> bool {
        let config_dir = get_config_dir();
        let config_file = get_config_file();
        pacman::install(PACKAGE_NAME);
        shell::execute(format!("mkdir -p {config_dir}"));
        fs::write(config_file, include_bytes!("../assets/terminator-config")).is_ok()
    }

    fn uninstall(&self) -> bool {
        let config_dir = get_config_dir();
        pacman::uninstall(PACKAGE_NAME);
        shell::execute(format!("rm -rf {config_dir}"))
    }

    fn is_installed(&self) -> bool {
        let config_file = get_config_file();
        let installed = pacman::is_installed(PACKAGE_NAME);
        let config_exists = Path::new(config_file.as_str()).exists();
        println!("installed: {installed}");
        println!("config_exists: {config_exists}");
        installed && config_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Terminator")
    }
}

fn get_config_dir() -> String {
    dirs::home_dir().unwrap()
        .join(".config")
        .join("terminator")
        .to_str().unwrap().to_string()
}

fn get_config_file() -> String {
    dirs::home_dir().unwrap()
        .join(".config")
        .join("terminator")
        .join("config")
        .to_str().unwrap().to_string()
}