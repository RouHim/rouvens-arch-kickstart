use std::thread;
use std::time::Duration;

use crate::shell;

pub fn install(package_name: &str) -> bool {
    let is_ok = shell::execute(format!("yay -Sy --noconfirm {package_name}"));

    while !is_installed(package_name) {
        println!("Waiting for package installation...");
        thread::sleep(Duration::from_millis(500));
    }

    is_ok
}

pub fn uninstall(package_name: &str) -> bool {
    let is_ok = shell::execute(format!("yay -Rs --noconfirm {package_name}"));

    while is_installed(package_name) {
        println!("Waiting for package uninstallation...");
        thread::sleep(Duration::from_millis(500));
    }

    is_ok
}

pub fn is_installed(package_name: &str) -> bool {
    shell::execute(format!("yay -Q {package_name}"))
}
