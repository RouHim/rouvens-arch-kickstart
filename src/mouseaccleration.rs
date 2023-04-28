use std::process::Command;

use crate::chaoticaur;
use crate::Feature;
use crate::pacman;
use crate::shell;
use crate::yay;

pub struct DisableMouseAcceleration {}

impl Feature for DisableMouseAcceleration {
    fn install(&self) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.mouse accel-profile 'flat'")
    }

    fn uninstall(&self) -> bool {
        shell::execute("gsettings set org.gnome.desktop.peripherals.mouse accel-profile 'default'")
    }

    fn is_installed(&self) -> bool {
        let output = shell::execute_with_output("gsettings get org.gnome.desktop.peripherals.mouse accel-profile");
        match output {
            Some(output) => output.contains("flat"),
            None => false,
        }
    }

    fn get_name(&self) -> String {
        String::from("Disable mouse acceleration")
    }
}