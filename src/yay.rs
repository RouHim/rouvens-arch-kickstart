use crate::shell;

pub fn install(package_name: &str) -> bool {
    shell::execute_as_user(format!("yay -Sy --noconfirm {package_name}").as_str())
}
pub fn uninstall(package_name: &str) -> bool {
    shell::execute_as_user(format!("yay -Rs --noconfirm {package_name}").as_str())
}

pub fn is_installed(package_name: &str) -> bool {
    shell::execute_as_user(format!("yay -Q {package_name}").as_str())
}
