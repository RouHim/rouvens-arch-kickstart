use crate::shell::RootShell;
use crate::{pacman, shell, Feature};
use std::thread;

#[derive(Clone)]
pub struct Docker {}

const SERVICE_NAME: &str = "docker.service";
const PACKAGE_NAME: &str = "docker";

impl Feature for Docker {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install(PACKAGE_NAME, root_shell);
        let username = shell::get_current_user();
        root_shell.execute(format!("usermod -aG docker {username}"));
        thread::sleep(std::time::Duration::from_millis(100));
        root_shell.execute(format!("systemctl enable {SERVICE_NAME}"));
        thread::sleep(std::time::Duration::from_millis(100));
        root_shell.execute(format!("systemctl start {SERVICE_NAME}"))
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall(PACKAGE_NAME, root_shell)
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
