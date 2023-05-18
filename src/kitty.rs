use std::path::PathBuf;
use std::thread;

use crate::shell::RootShell;
use crate::{filesystem, pacman, shell, Feature};

pub struct Kitty {}

const PACKAGE_NAME: &str = "kitty";

impl Feature for Kitty {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Install kitty
        let ok = pacman::install(PACKAGE_NAME, root_shell);

        // Ensure kitty config directory exists
        let kitty_config_path = get_kitty_config_file();
        let kitty_config_dir = kitty_config_path.parent().unwrap();
        shell::execute(format!("mkdir -p {}", kitty_config_dir.to_str().unwrap()));

        // Download kitty config
        filesystem::download_file(
            "https://sw.kovidgoyal.net/kitty/_downloads/433dadebd0bf504f8b008985378086ce/kitty.conf",
            get_kitty_config_file(),
        );

        configure_kitty();

        // Make kitty-open.desktop readonly
        // This is necessary because kitty-open.desktop is overwritten by kitty
        let file = PathBuf::from("/usr/share/applications/kitty-open.desktop");
        root_shell.execute(format!("chattr -i {}", file.to_str().unwrap()));
        thread::sleep(std::time::Duration::from_millis(100));

        // Write "[Desktop Entry]" to kitty-open.desktop
        root_shell.execute(format!(
            "echo \"[Desktop Entry]\" > {}",
            file.to_str().unwrap()
        ));

        root_shell.execute(format!("chattr +i {}", file.to_str().unwrap()));

        ok
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        let file = PathBuf::from("/usr/share/applications/kitty-open.desktop");
        root_shell.execute(format!("chattr -i {}", file.to_str().unwrap()));

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

    // Set kitty config transparent background to 90%
    filesystem::replace_string_in_file(
        &config_dir,
        "# wayland_titlebar_color system",
        "wayland_titlebar_color #555555",
    );
}

fn get_kitty_config_file() -> PathBuf {
    shell::user_home_dir_path()
        .join(".config")
        .join("kitty")
        .join("kitty.conf")
}
