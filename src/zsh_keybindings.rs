use lazy_static::lazy_static;

use crate::shell::RootShell;
use crate::{zshrc, Feature};

pub struct ZshCommonKeyBindings {}

lazy_static! {
    static ref KEY_BINDS: Vec<String> = vec![
        "bindkey -e".to_string(),
        r#"bindkey "^[[3~" delete-char"#.to_string(),
        r#"bindkey "^[[5~" beginning-of-buffer-or-history"#.to_string(),
        r#"bindkey "^[[6~" end-of-buffer-or-history"#.to_string(),
        r#"bindkey "^[[H" beginning-of-line"#.to_string(),
        r#"bindkey "^[[F" end-of-line"#.to_string(),
        r#"bindkey "^[[1;5C" forward-word"#.to_string(),
        r#"bindkey "^[[1;5D" backward-word"#.to_string(),
    ];
}

impl Feature for ZshCommonKeyBindings {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        zshrc::add_line("\n");
        zshrc::add_line("# Common Zsh Key Bindings");
        KEY_BINDS.iter().for_each(|line| {
            zshrc::add_line(line);
        });
        zshrc::add_line("");

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        zshrc::remove_line("# Common Zsh Key Bindings");
        KEY_BINDS.iter().for_each(|line| {
            zshrc::remove_line(line);
        });

        true
    }

    fn is_installed(&self) -> bool {
        KEY_BINDS.iter().all(|line| zshrc::line_exists(line))
    }

    fn get_name(&self) -> String {
        "Common Zsh Keybindings".to_string()
    }
}
