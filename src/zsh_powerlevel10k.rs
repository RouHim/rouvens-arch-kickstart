use std::path::Path;

use crate::shell::RootShell;
use crate::{shell, zsh_default_shell, zshrc, Feature};

pub struct ZshPowerLevel10k {}

const ZSHRC_CONFIG_LINE: &str = "source ~/powerlevel10k/powerlevel10k.zsh-theme";
const GIT_REPO: &str = "https://github.com/romkatv/powerlevel10k.git";

impl Feature for ZshPowerLevel10k {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Make sure ZSH shell is default
        let zsh = zsh_default_shell::ZshDefaultShell {};
        if !zsh.is_installed() {
            zsh.install(root_shell);
        }

        // Install p10k
        let local_folder = get_local_folder();
        let clone_command = format!("git clone --depth=1 {GIT_REPO} {local_folder}");
        let clone_ok = shell::execute(clone_command.as_str());

        let zsh_ok = zshrc::add_line(ZSHRC_CONFIG_LINE);

        clone_ok && zsh_ok
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        let local_folder = get_local_folder();
        let remove_local_folder_ok = root_shell.execute(format!("rm -rf {local_folder}"));

        let remove_zshrc_line_ok = zshrc::remove_line(ZSHRC_CONFIG_LINE);

        remove_local_folder_ok && remove_zshrc_line_ok
    }

    fn is_installed(&self) -> bool {
        let local_folder = get_local_folder();
        let local_folder_exists = Path::new(&local_folder).exists();
        let zshrc_config_line_exists = zshrc::line_exists(ZSHRC_CONFIG_LINE);

        local_folder_exists && zshrc_config_line_exists
    }

    fn get_name(&self) -> String {
        String::from("Install Zsh Powerlevel10k")
    }
}

fn get_local_folder() -> String {
    shell::user_home_dir_path()
        .join("powerlevel10k")
        .to_str()
        .unwrap()
        .to_string()
}
