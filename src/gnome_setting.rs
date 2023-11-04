use crate::shell::RootShell;
use crate::{shell, Feature};

#[derive(Clone)]
pub struct GnomeSetting {
    pub name: &'static str,
    pub value: &'static str,
    pub default_value: &'static str,
    pub description: &'static str,
}

impl Feature for GnomeSetting {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        shell::execute(format!("gsettings set {} {}", self.name, self.value))
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        shell::execute(format!(
            "gsettings set {} {}",
            self.name, self.default_value
        ))
    }

    fn is_installed(&self) -> bool {
        let output = shell::execute_with_output(format!("gsettings get {}", self.name));
        output.contains(self.value)
    }

    fn get_name(&self) -> String {
        self.description.to_string()
    }
}
