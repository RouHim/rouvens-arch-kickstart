use std::path::Path;

use crate::{shell, zshrc, Feature};

pub struct ZshPowerLevel10k {}

const ZSHRC_CONFIG_LINE: &str = "source ~/powerlevel10k/powerlevel10k.zsh-theme";
const GIT_REPO: &str = "https://github.com/romkatv/powerlevel10k.git";

impl Feature for ZshPowerLevel10k {
    fn install(&self) -> bool {
        let local_folder = get_local_folder();
        let clone_command = format!("git clone --depth=1 {GIT_REPO} {local_folder}");
        let clone_ok = shell::execute(clone_command);

        let zsh_ok = zshrc::add_line(ZSHRC_CONFIG_LINE);

        clone_ok && zsh_ok
    }

    fn uninstall(&self) -> bool {
        let local_folder = get_local_folder();
        let remove_local_folder_command = format!("rm -rf {local_folder}");
        let remove_local_folder_ok = shell::execute(remove_local_folder_command);

        let remove_zshrc_line_ok = zshrc::remove_line(ZSHRC_CONFIG_LINE);

        remove_local_folder_ok && remove_zshrc_line_ok
    }

    fn is_installed(&self) -> bool {
        let local_folder = get_local_folder();
        let local_folder_exists = Path::new(&local_folder).exists();
        let zshrc_config_line_exists = zshrc::line_exists(ZSHRC_CONFIG_LINE);

        println!("local_folder_exists: {local_folder_exists}");
        println!("zshrc_config_line_exists: {zshrc_config_line_exists}");

        local_folder_exists && zshrc_config_line_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Zsh Powerlevel10k")
    }
}

fn get_local_folder() -> String {
    dirs::home_dir()
        .unwrap()
        .join("powerlevel10k")
        .to_str()
        .unwrap()
        .to_string()
}
