use std::fs;
use std::path::Path;

use crate::{Feature, pacman, shell};

pub struct Alacritty {}

const PACKAGE_NAME: &str = "alacritty tmux";

impl Feature for Alacritty {
    fn install(&self) -> bool {
        let alacritty_config_dir = get_alacritty_config_dir();
        let alacritty_config_file = get_alacritty_config_file();
        let tmux_config_file = get_tmux_config_file();

        // Install Alacritty and Tmux
        let package_status = pacman::install(PACKAGE_NAME);

        // Ensure the Alacritty config directory exists
        shell::execute(format!("mkdir -p {alacritty_config_dir}"));

        // Write the Alacritty config file
        let config_status_alacritty = fs::write(
            alacritty_config_file,
            include_bytes!("../assets/alacritty-config"),
        )
            .is_ok();
        // Own the file as the current user of the HOME folder
        shell::execute(format!("chown -R $SUDO_USER {alacritty_config_dir}"));

        let config_status_tmux =
            fs::write(&tmux_config_file, include_bytes!("../assets/tmux-config")).is_ok();
        // Own the file as the current user of the HOME folder
        shell::execute(format!("chown -R $SUDO_USER {tmux_config_file}"));

        package_status && config_status_alacritty && config_status_tmux
    }

    fn uninstall(&self) -> bool {
        let alacritty_config_dir = get_alacritty_config_dir();
        let tmux_config_file = get_tmux_config_file();
        pacman::uninstall(PACKAGE_NAME);
        let alacritty_status = shell::execute(format!("rm -rf {alacritty_config_dir}"));
        let tmux_status = shell::execute(format!("rm -rf {tmux_config_file}"));
        alacritty_status && tmux_status
    }

    fn is_installed(&self) -> bool {
        let alacritty_config_file = get_alacritty_config_file();
        let tmux_config_file = get_tmux_config_file();
        let installed = pacman::is_installed(PACKAGE_NAME);
        let alacritty_config_exists = Path::new(alacritty_config_file.as_str()).exists();
        let tmux_config_exists = Path::new(tmux_config_file.as_str()).exists();
        installed && alacritty_config_exists && tmux_config_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Alacritty and Tmux")
    }
}

fn get_alacritty_config_dir() -> String {
    shell::sudo_user_home_dir()
        .join(".config")
        .join("alacritty")
        .to_str()
        .unwrap()
        .to_string()
}

fn get_alacritty_config_file() -> String {
    get_alacritty_config_dir() + "/alacritty.yml"
}

fn get_tmux_config_file() -> String {
    shell::sudo_user_home_dir()
        .join(".tmux.conf")
        .to_str()
        .unwrap()
        .to_string()
}
