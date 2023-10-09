use crate::shell::RootShell;
use crate::{shell, Feature};

#[derive(Clone)]
pub struct PeriodicTRIM {}

impl Feature for PeriodicTRIM {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Check if the fstrim.timer is present:
        let timer_status = shell::execute_with_output("systemctl list-timers");
        if timer_status.contains("fstrim.timer") {
            root_shell.execute("systemctl enable fstrim.timer");
            root_shell.execute("systemctl start fstrim.timer");
        }

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        root_shell.execute("systemctl stop fstrim.timer");
        root_shell.execute("systemctl disable fstrim.timer")
    }

    fn is_installed(&self) -> bool {
        shell::execute("systemctl status fstrim.timer")
    }

    fn get_name(&self) -> String {
        String::from("Setup PeriodicTRIM")
    }
}
