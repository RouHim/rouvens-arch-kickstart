use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

#[derive(Clone)]
pub struct Bluetooth {}

const SERVICE_NAME: &str = "bluetooth.service";

impl Feature for Bluetooth {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install("bluez bluez-utils", root_shell);
        root_shell.execute(format!("systemctl enable {SERVICE_NAME}"));
        root_shell.execute(format!("systemctl start {SERVICE_NAME}"))
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        root_shell.execute(format!("systemctl stop {SERVICE_NAME}"));
        root_shell.execute(format!("systemctl disable {SERVICE_NAME}"))
    }

    fn is_installed(&self) -> bool {
        shell::execute(format!("systemctl is-enabled -q {SERVICE_NAME}"))
    }

    fn get_name(&self) -> String {
        String::from("Setup Bluetooth service")
    }
}
