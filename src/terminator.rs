use std::fs;
use std::path::Path;

use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

#[derive(Clone)]
pub struct Terminator {}

const PACKAGE_NAME: &str = "terminator";

impl Feature for Terminator {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        let config_dir = get_config_dir();
        let config_file = get_config_file();

        // Install terminator
        pacman::install(PACKAGE_NAME, root_shell);

        // Copy config file
        shell::execute(format!("mkdir -p {config_dir}").as_str());
        fs::write(config_file, include_bytes!("../assets/terminator-config")).is_ok()
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Uninstall terminator
        pacman::uninstall(PACKAGE_NAME, root_shell);

        // Remove config file
        let config_dir = get_config_dir();
        root_shell.execute(format!("rm -rf {config_dir}"))
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
