use std::fs;
use std::path::PathBuf;

use crate::shell::RootShell;
use crate::yay::is_installed;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct GnomeDraculaGtkTheme {}

const THEME_PACKAGE_NAME: &str = "dracula-gtk-theme-git";
const ICONS_PACKAGE_NAME: &str = "dracula-icons-git";
const CURSOR_PACKAGE_NAME: &str = "dracula-cursors-git";
const GTK_THEME_INSTALL_PATH: &str = "/usr/share/themes/Dracula/gnome-shell";
const GTK_THEME_INSTALL_PATH_V40: &str = "/usr/share/themes/Dracula/gnome-shell/v40";

impl Feature for GnomeDraculaGtkTheme {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Then install the GTK theme and icons
        pacman::install(THEME_PACKAGE_NAME, root_shell);
        pacman::install(ICONS_PACKAGE_NAME, root_shell);
        pacman::install(CURSOR_PACKAGE_NAME, root_shell);

        // Then make sure GTK_THEME Dracula is set in /etc/environment using root shell
        if !fs::read_to_string("/etc/environment")
            .unwrap()
            .contains("GTK_THEME=Dracula")
        {
            root_shell.execute("echo 'GTK_THEME=Dracula' >> /etc/environment");
        }

        // Apply the gnome shell v40 themes to default styling
        if fs::metadata(GTK_THEME_INSTALL_PATH_V40).is_ok() {
            root_shell.execute(format!(
                "cp -r {GTK_THEME_INSTALL_PATH_V40}/* {GTK_THEME_INSTALL_PATH}"
            ));
        }

        // Activate dracula theme everywhere
        root_shell.execute("gsettings set org.gnome.desktop.interface gtk-theme 'Dracula'");
        root_shell.execute("gsettings set org.gnome.desktop.wm.preferences theme 'Dracula'");
        root_shell.execute("gsettings set org.gnome.shell.extensions.user-theme name 'Dracula'");
        root_shell.execute("gsettings set org.gnome.desktop.interface icon-theme 'Dracula'");
        root_shell
            .execute("gsettings set org.gnome.desktop.interface cursor-theme 'Dracula-cursors'");

        GnomeDraculaGtkTheme::is_installed(self)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // remove the Arc GTK theme
        pacman::uninstall(THEME_PACKAGE_NAME, root_shell);
        pacman::uninstall(ICONS_PACKAGE_NAME, root_shell);
        pacman::uninstall(CURSOR_PACKAGE_NAME, root_shell);

        // remove the GTK_THEME Dracula from /etc/environment using root shell
        root_shell.execute("sed -i '/GTK_THEME=Dracula/d' /etc/environment");

        !GnomeDraculaGtkTheme::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        let is_installed = is_installed(THEME_PACKAGE_NAME)
            && is_installed(ICONS_PACKAGE_NAME)
            && is_installed(CURSOR_PACKAGE_NAME);

        // Check if the GTK_THEME Dracula is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(environment_file).unwrap();
        let is_gtk_theme_arc_dark_set = environment_file_contents.contains("GTK_THEME=Dracula");

        is_installed && is_gtk_theme_arc_dark_set
    }

    fn get_name(&self) -> String {
        String::from("Gnome Dracula GTK Theme")
    }
}
