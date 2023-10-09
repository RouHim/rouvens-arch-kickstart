use crate::shell::RootShell;
use crate::{shell, Feature, pacman};

#[derive(Clone)]
pub struct Splashscreen {}

impl Feature for Splashscreen {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        //TODO
        // Install plymouth
        pacman::install("plymouth", root_shell);

        // append quiet and splash to the kernel parameters.
        root_shell.execute("sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT=\"\\(.*\\)\"/GRUB_CMDLINE_LINUX_DEFAULT=\"\\1 quiet splash\"/' /etc/default/grub");

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        true
    }

    fn is_installed(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        String::from("Setup Splashscreen")
    }
}
