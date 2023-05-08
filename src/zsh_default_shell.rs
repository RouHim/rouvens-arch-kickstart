use crate::{shell, Feature};

pub struct ZshDefaultShell {}

impl Feature for ZshDefaultShell {
    fn install(&self) -> bool {
        shell::execute_as_root("chsh -s $(which zsh)")
    }

    fn uninstall(&self) -> bool {
        shell::execute_as_root("chsh -s $(which bash)")
    }

    fn is_installed(&self) -> bool {
        shell::execute_as_root(
            r#"if [ "$(echo $SHELL)" = "$(which zsh)" ]; then exit 0; else exit 1; fi"#,
        )
    }

    fn get_name(&self) -> String {
        String::from("Set ZSH as default shell")
    }
}
