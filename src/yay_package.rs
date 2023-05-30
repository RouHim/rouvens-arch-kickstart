use crate::shell::RootShell;
use crate::{yay, Feature};

#[derive(Clone)]
pub struct YayPackage {
    pub package_name: &'static str,
    pub description: &'static str,
}

impl Feature for YayPackage {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        yay::install(self.package_name)
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        yay::uninstall(self.package_name)
    }

    fn is_installed(&self) -> bool {
        yay::is_installed(self.package_name)
    }

    fn get_name(&self) -> String {
        self.description.to_string()
    }
}
