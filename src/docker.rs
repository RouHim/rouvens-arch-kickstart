use crate::{pacman, shell, Feature};

pub struct Docker {}

const SERVICE_NAME: &str = "bluetooth.service";
const PACKAGE_NAME: &str = "docker";

impl Feature for Docker {
    fn install(&self) -> bool {
        pacman::install(PACKAGE_NAME);
        shell::execute(format!("usermod -aG docker $SUDO_USER"));
        shell::execute(format!("systemctl enable {SERVICE_NAME}"));
        shell::execute(format!("systemctl start {SERVICE_NAME}"))
    }

    fn uninstall(&self) -> bool {
        pacman::uninstall(PACKAGE_NAME);
        shell::execute(format!("systemctl disable {SERVICE_NAME}"))
    }

    fn is_installed(&self) -> bool {
        let package_installed = pacman::is_installed(PACKAGE_NAME);
        let status_active = shell::execute(format!("systemctl status {SERVICE_NAME}"));

        package_installed && status_active
    }

    fn get_name(&self) -> String {
        String::from("Setup Docker")
    }
}
