use crate::shell;

pub fn install(package_name: &str) -> bool {
    shell::execute(format!("yay -Sy --noconfirm {package_name}"))
}

pub fn uninstall(package_name: &str) -> bool {
    shell::execute(format!("yay -Rs --noconfirm {package_name}"))
}

pub fn is_installed(package_name: &str) -> bool {
    shell::execute(format!("yay -Q {package_name}"))
}
