use crate::shell::RootShell;
use crate::{pacman, shell, Feature};

#[derive(Clone)]
pub struct Splashscreen {}

impl Feature for Splashscreen {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        pacman::install("plymouth mkinitcpio", root_shell);

        // Add plymouth to the HOOKS in mkinitcpio.conf
        root_shell
            .execute("sed -i 's/HOOKS=\"\\(.*\\)\"/HOOKS=\"\\1 plymouth\"/' /etc/mkinitcpio.conf");

        // Check if system runs grub, configure grub
        if shell::execute("which grub-mkconfig") {
            pacman::install("update-grub", root_shell);

            // append quiet and splash to the kernel parameters.
            root_shell.execute("sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT=\"\\(.*\\)\"/GRUB_CMDLINE_LINUX_DEFAULT=\"\\1 quiet splash\"/' /etc/default/grub");
        }

        // run sudo mkinitcpio -g linux
        root_shell.execute("mkinitcpio -g linux");

        // if grub, also run update grub
        if shell::execute("which grub-mkconfig") {
            root_shell.execute("update-grub");
        }

        true
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        pacman::uninstall("plymouth", root_shell);

        // Remove plymouth from the HOOKS in mkinitcpio.conf
        root_shell.execute("sed -i 's/ plymouth//' /etc/mkinitcpio.conf");

        // Check if system runs grub, configure grub
        if shell::execute("which grub-mkconfig") {
            // remove quiet and splash from the kernel parameters.
            root_shell.execute("sed -i 's/ quiet splash//' /etc/default/grub");
        }

        // run sudo mkinitcpio -g linux
        root_shell.execute("mkinitcpio -g linux");

        // if grub, also run update grub
        if shell::execute("which grub-mkconfig") {
            root_shell.execute("update-grub");
        }

        true
    }

    fn is_installed(&self) -> bool {
        // Check if plymouth is installed
        let plymouth_installed = shell::execute("which plymouth");

        // Check if plymouth is in the HOOKS in mkinitcpio.conf
        let plymouth_in_mkinitcpio = shell::execute("grep -q plymouth /etc/mkinitcpio.conf");

        // Check if system runs grub, configure grub

        let grub_configured = if shell::execute("which grub-mkconfig") {
            shell::execute("grep -q 'quiet splash' /etc/default/grub")
        } else {
            true
        };

        plymouth_installed && plymouth_in_mkinitcpio && grub_configured
    }

    fn get_name(&self) -> String {
        String::from("Setup Splashscreen")
    }
}
