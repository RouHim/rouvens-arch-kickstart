use crate::{Feature, pacman, shell};

pub struct Docker {}

const SERVICE_NAME: &str = "docker.service";
const PACKAGE_NAME: &str = "docker";

impl Feature for Docker {
    fn install(&self) -> bool {
        pacman::install(PACKAGE_NAME);
        shell::execute_as_root("usermod -aG docker $SUDO_USER");
        shell::execute_as_root(format!("systemctl enable {SERVICE_NAME}"));
        shell::execute_as_root(format!("systemctl start {SERVICE_NAME}"))
    }

    fn uninstall(&self) -> bool {
        shell::execute_as_root(format!("systemctl stop {SERVICE_NAME}"));
        shell::execute_as_root(format!("systemctl disable {SERVICE_NAME}"));
        pacman::uninstall(PACKAGE_NAME)
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
