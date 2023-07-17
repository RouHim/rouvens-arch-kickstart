use crate::shell::RootShell;
use crate::{shell, Feature};

#[derive(Clone)]
pub struct HibernateWhenLidClosed {}

impl Feature for HibernateWhenLidClosed {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // Enable hibernation when lid is closed
        root_shell.execute("sed -i 's/#HandleLidSwitch=suspend/HandleLidSwitch=hibernate/g' /etc/systemd/logind.conf");
        root_shell.execute("sed -i 's/#HandleLidSwitchExternalPower=suspend/HandleLidSwitchExternalPower=hibernate/g' /etc/systemd/logind.conf")
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // Disable hibernation when lid is closed (default is suspend)
        root_shell.execute("sed -i 's/HandleLidSwitch=hibernate/#HandleLidSwitch=suspend/g' /etc/systemd/logind.conf");
        root_shell.execute("sed -i 's/HandleLidSwitchExternalPower=hibernate/#HandleLidSwitchExternalPower=suspend/g' /etc/systemd/logind.conf")
    }

    fn is_installed(&self) -> bool {
        // Check if hibernation is enabled when lid is closed
        let output =
            shell::execute("cat /etc/systemd/logind.conf | grep HandleLidSwitch=hibernate");
        let output2 = shell::execute(
            "cat /etc/systemd/logind.conf | grep HandleLidSwitchExternalPower=hibernate",
        );
        output && output2
    }

    fn get_name(&self) -> String {
        String::from("Hibernate when lid closed")
    }
}
