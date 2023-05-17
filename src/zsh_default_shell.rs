use crate::shell::RootShell;

use crate::yay::is_installed;
use crate::{pacman, shell, zshrc, Feature};

pub struct ZshDefaultShell {}

const ZSH_CONFIG: &str = r#"

# Enable aliases
alias ls='ls -GFh'
alias ll='ls -l'
alias la='ls -a'
alias l='ls -CF'

# Configure the editor
export EDITOR='nano'

# Store the history in a file
HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
"#;

impl Feature for ZshDefaultShell {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        if !pacman::is_installed("zsh") {
            pacman::install("zsh", root_shell);
        }

        let username = shell::get_current_user();
        shell::execute(format!("pkexec chsh --shell $(which zsh) {username}"));
        zshrc::add_line(ZSH_CONFIG);

        is_installed("zsh")
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        let username = shell::get_current_user();
        zshrc::remove_line(ZSH_CONFIG);
        shell::execute(format!("pkexec chsh --shell $(which bash) {username}"))
    }

    fn is_installed(&self) -> bool {
        let username = shell::get_current_user();
        shell::execute(format!("cat /etc/passwd | grep {username} | grep /zsh"))
    }

    fn get_name(&self) -> String {
        String::from("Set ZSH as default shell")
    }
}
