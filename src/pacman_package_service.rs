use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

#[derive(Clone)]
pub struct PacmanPackageService {
    pub package_name: &'static str,
    pub service_name: &'static str,
    pub description: &'static str,
}

impl Feature for PacmanPackageService {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        let package_status = pacman::install(self.package_name, root_shell);
        root_shell.execute(format!("systemctl enable {}", self.service_name));
        root_shell.execute(format!("systemctl start {}", self.service_name));
        let service_status = is_service_enabled(self.service_name);
        package_status && service_status
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        root_shell.execute(format!("systemctl stop {}", self.service_name));
        root_shell.execute(format!("systemctl disable {}", self.service_name));
        let service_status = is_service_enabled(self.service_name);
        let package_status = pacman::uninstall(self.package_name, root_shell);
        package_status && service_status
    }

    fn is_installed(&self) -> bool {
        let package_status = pacman::is_installed(self.package_name);
        let service_status = is_service_enabled(self.service_name);
        package_status && service_status
    }

    fn get_name(&self) -> String {
        self.description.to_string()
    }
}

fn is_service_enabled(service_name: &str) -> bool {
    shell::execute(format!("systemctl is-enabled -q {}", service_name))
}
