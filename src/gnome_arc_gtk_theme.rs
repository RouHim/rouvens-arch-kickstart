use std::fs;

use std::path::PathBuf;

use crate::shell::RootShell;
use crate::yay::is_installed;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct GnomeArcGtkTheme {}

const PACKAGE_NAME: &str = "arc-gtk-theme";

impl Feature for GnomeArcGtkTheme {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // First remove the EndavourOs default theme (arc-gtk-theme-eos) if present
        pacman::uninstall("arc-gtk-theme-eos", root_shell);

        // Then install the Arc GTK theme
        pacman::install(PACKAGE_NAME, root_shell);

        // Then make sure GTK_THEME Arc-Dark is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        if !environment_file_contents.contains("GTK_THEME=Arc-Dark") {
            fs::write(&environment_file, "GTK_THEME=Arc-Dark").unwrap();
        }

        GnomeArcGtkTheme::is_installed(self)
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        // remove the Arc GTK theme
        pacman::uninstall(PACKAGE_NAME, _root_shell);

        // remove the GTK_THEME Arc-Dark from /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        if environment_file_contents.contains("GTK_THEME=Arc-Dark") {
            fs::write(&environment_file, "GTK_THEME=").unwrap();
        }

        !GnomeArcGtkTheme::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        let is_installed = is_installed(PACKAGE_NAME);

        // Check if the GTK_THEME Arc-Dark is set in /etc/environment
        let environment_file = PathBuf::from("/etc/environment");
        let environment_file_contents = fs::read_to_string(&environment_file).unwrap();
        let is_gtk_theme_arc_dark_set = environment_file_contents.contains("GTK_THEME=Arc-Dark");

        is_installed && is_gtk_theme_arc_dark_set
    }

    fn get_name(&self) -> String {
        String::from("Gnome Arc GTK Theme")
    }
}
