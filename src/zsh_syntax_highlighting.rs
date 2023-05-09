use crate::shell::RootShell;
use crate::{pacman, zshrc, Feature};

pub struct ZshSyntaxHighlighting {}

const PACKAGE_NAME: &str = "zsh-syntax-highlighting";
const ZSHRC_CONFIG: &str =
    "source /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh";

impl Feature for ZshSyntaxHighlighting {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Install package using pacman
        let pacman_is_ok = pacman::install(PACKAGE_NAME, root_shell);

        // Source in .zshrc
        let zsh_ok = zshrc::add_line(ZSHRC_CONFIG);

        pacman_is_ok && zsh_ok
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Uninstall using pacman
        let pacman_is_ok = pacman::uninstall(PACKAGE_NAME, root_shell);

        // Remove from zshrc
        let zsh_ok = zshrc::remove_line(ZSHRC_CONFIG);

        pacman_is_ok && zsh_ok
    }

    fn is_installed(&self) -> bool {
        // Check if package is installed
        let is_installed = pacman::is_installed(PACKAGE_NAME);

        // Check if import is present
        let zsh_ok = zshrc::line_exists(ZSHRC_CONFIG);

        is_installed && zsh_ok
    }

    fn get_name(&self) -> String {
        String::from("Install Zsh Syntax Highlighting")
    }
}
