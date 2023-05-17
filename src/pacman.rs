use std::thread;
use std::time::Duration;

use crate::shell;
use crate::shell::RootShell;

pub fn install(package_name: &str, root_shell: &mut RootShell) -> bool {
    let is_ok = root_shell.execute(format!("pacman -Sy --noconfirm {package_name}"));

    while !is_installed(package_name) {
        println!("Waiting for package installation...");
        thread::sleep(Duration::from_millis(500));
    }

    is_ok
}

pub fn uninstall(package_name: &str, root_shell: &mut RootShell) -> bool {
    let is_ok = root_shell.execute(format!("pacman -Rs --noconfirm {package_name}"));

    while is_installed(package_name) {
        println!("Waiting for package uninstallation...");
        thread::sleep(Duration::from_millis(500));
    }

    is_ok
}

pub fn is_installed(package_name: &str) -> bool {
    shell::execute(format!("pacman -Q {package_name}"))
}
