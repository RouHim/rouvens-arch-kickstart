use std::fs::OpenOptions;
use std::io::{Write};
use std::path::{PathBuf};


use crate::shell::RootShell;
use crate::{filesystem, pacman, shell, Feature};

pub struct Kitty {}

const PACKAGE_NAME: &str = "kitty";

impl Feature for Kitty {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Install kitty
        let ok = pacman::install(PACKAGE_NAME, root_shell);

        configure_kitty();

        // Write "[Desktop Entry]" to /usr/share/applications/kitty-open.desktop and make it readonly
        // This avoids kitty from being the default for opening files and folders
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/usr/share/applications/kitty-open.desktop")
            .unwrap();
        file.write_all(b"[Desktop Entry]").unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_readonly(true);
        file.set_permissions(perms).unwrap();

        ok
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/usr/share/applications/kitty-open.desktop")
            .unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_readonly(true);
        file.set_permissions(perms).unwrap();

        // Uninstall kitty
        pacman::uninstall(PACKAGE_NAME, root_shell)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Install Kitty")
    }
}

fn configure_kitty() {
    let config_dir = get_kitty_config_file();

    filesystem::replace_string_in_file(
        &config_dir,
        "# map kitty_mod+] next_window",
        "map kitty_mod+down next_window",
    );

    filesystem::replace_string_in_file(
        &config_dir,
        "# map kitty_mod+[ previous_window",
        "map kitty_mod+up previous_window",
    );

    filesystem::replace_string_in_file(
        &config_dir,
        "# tab_bar_style powerline",
        "tab_bar_style powerline",
    );

    // Set kitty config transparent background to 90%
    filesystem::replace_string_in_file(
        &config_dir,
        "# background_opacity 1.0",
        "background_opacity 0.9",
    );
}

fn get_kitty_config_file() -> PathBuf {
    shell::user_home_dir_path()
        .join(".config")
        .join("kitty")
        .join("kitty.conf")
}
