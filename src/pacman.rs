use crate::shell;
use crate::shell::RootShell;

pub fn install(package_name: &str, root_shell: &mut RootShell) -> bool {
    root_shell.execute(format!("pacman -Sy --noconfirm {package_name}"))
}

pub fn uninstall(package_name: &str, root_shell: &mut RootShell) -> bool {
    root_shell.execute(format!("pacman -Rs --noconfirm {package_name}"))
}

pub fn is_installed(package_name: &str) -> bool {
    shell::execute(format!("pacman -Q {package_name}"))
}
