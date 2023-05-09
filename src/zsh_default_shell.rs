use crate::shell::RootShell;
use crate::{Feature, shell};

pub struct ZshDefaultShell {}

impl Feature for ZshDefaultShell {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        root_shell.execute("chsh -s $(which zsh)")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
         root_shell.execute("chsh -s $(which bash)")
    }

    fn is_installed(&self) -> bool {
        shell::execute(
            r#"if [ "$(echo $SHELL)" = "$(which zsh)" ]; then exit 0; else exit 1; fi"#,
        )
    }

    fn get_name(&self) -> String {
        String::from("Set ZSH as default shell")
    }
}
