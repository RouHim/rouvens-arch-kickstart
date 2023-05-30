use crate::chaotic_aur::ChaoticAur;
use crate::shell::RootShell;
use dyn_clone::DynClone;

use std::boxed::Box;

mod bluetooth;
mod chaotic_aur;
mod docker;
mod filesystem;
mod fish_default_shell;
mod gnome_app_indicator;
mod gnome_blur_my_shell;
mod gnome_dark_mode;
mod gnome_dash_to_panel;
mod gnome_just_perfection;
mod gnome_mouse_acceleration;
mod gnome_shortcuts;
mod gnome_system_monitor;
mod gnome_tap_to_click;
mod kitty;
mod pacman;
mod pacman_package;
mod pacman_pamac;
mod rust;
mod shell;
mod terminator;
mod ui;
mod yay;
mod yay_package;

pub trait Feature: DynClone {
    fn install(&self, root_shell: &mut RootShell) -> bool;
    fn uninstall(&self, root_shell: &mut RootShell) -> bool;
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
    fn is_group_element(&self) -> bool {
        false
    }
}

fn main() {
    // Ensure that the user is running the script as non root
    // And the home directory is not root
    // And the user is running an arch based distro
    ensure_non_root_privileges();
    ensure_arch_based_distro();

    // Request root shell
    let mut root_shell: RootShell = RootShell::new().unwrap();

    ensure_yay_is_installed(&mut root_shell);
    ensure_chaotic_aur_is_installed(&mut root_shell);

    let features: Vec<Box<dyn Feature>> = vec![
        // Pacman
        Box::new(pacman_pamac::PacmanPamac {}),
        // System
        Box::new(FeatureGroup {
            name: "System".to_string(),
        }),
        Box::new(bluetooth::Bluetooth {}),
        Box::new(docker::Docker {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "noto-fonts-emoji",
            description: "Install emoji support",
        }),
        // Shell
        Box::new(FeatureGroup {
            name: "Shell".to_string(),
        }),
        Box::new(fish_default_shell::FishDefaultShell {}),
        Box::new(kitty::Kitty {}),
        Box::new(terminator::Terminator {}),
        // Gnome
        Box::new(FeatureGroup {
            name: "Gnome".to_string(),
        }),
        Box::new(gnome_dark_mode::GnomeDarkMode {}),
        Box::new(gnome_tap_to_click::GnomeTapToClick {}),
        Box::new(gnome_mouse_acceleration::GnomeDisableMouseAcceleration {}),
        Box::new(gnome_shortcuts::GnomeKeyboardShortcuts {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gnome-browser-connector",
            description: "Install Gnome Browser connector",
        }),
        Box::new(gnome_system_monitor::GnomeShellExtensionSystemMonitor {}),
        Box::new(gnome_dash_to_panel::GnomeShellExtensionDashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(gnome_blur_my_shell::GnomeShellExtensionBlurMyShell {}),
        Box::new(gnome_just_perfection::GnomeShellExtensionJustPerfection {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "arc-gtk-theme",
            description: "Install GTK Arc Theme",
        }),
        // Apps
        Box::new(FeatureGroup {
            name: "Common Packages".to_string(),
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "fwupd",
            description: "Install firmware updater",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "topgrade",
            description: "Install topgrade",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "htop",
            description: "Install htop",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "btop",
            description: "Install btop",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gparted",
            description: "Install gparted",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "timeshift",
            description: "Install timeshift",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "sublime-text-4",
            description: "Install Sublime",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "bitwarden",
            description: "Install Bitwarden",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "mc",
            description: "Install Midnight commander",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "solaar",
            description: "Install Solaar (Logitech)",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "micro",
            description: "Install Micro",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "thunderbird",
            description: "Install Thunderbird",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "signal-desktop",
            description: "Install Signal",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "usbimager",
            description: "Install USB Imager",
        }),
        // Development
        Box::new(FeatureGroup {
            name: "Software Development".to_string(),
        }),
        Box::new(rust::Rust {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "vscodium",
            description: "Install VS Codium",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "python",
            description: "Install Python",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "jdk-openjdk",
            description: "Install OpenJDK",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "maven",
            description: "Install Maven",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "intellij-idea-ultimate-edition intellij-idea-ultimate-edition-jre",
            description: "Install intelliJ IDEA Ultimate",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "qemu-user-static",
            description: "Install QEMU static",
        }),
        // Networking
        Box::new(FeatureGroup {
            name: "Networking".to_string(),
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "wireguard-tools",
            description: "Install Wireguard",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "wireless_tools",
            description: "Install Wireless tools",
        }),
        // Gaming
        Box::new(FeatureGroup {
            name: "Gaming".to_string(),
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "steam steam-native-runtime",
            description: "Install Steam",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "lutris gamemode lib32-gamemode innoextract gvfs lib32-vkd3d lib32-vulkan-icd-loader vkd3d vulkan-icd-loader vulkan-tools wine winetricks",
            description: "Install Lutris",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "protonplus",
            description: "Install Proton Plus",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "mangohud",
            description: "Install MangoHud",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "piper",
            description: "Install Piper",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "openrgb",
            description: "Install OpenRGB",
        }),
        Box::new(yay_package::YayPackage {
            package_name: "lug-helper",
            description: "Install Star Citizen LUG Helper",
        }),
    ];

    ui::show(root_shell, features);
}

fn ensure_chaotic_aur_is_installed(root_shell: &mut RootShell) {
    let aur = ChaoticAur {};
    if !aur.is_installed() {
        println!("✔️ Installing Chaotic AUR");
        aur.install(root_shell);
    } else {
        println!("✔️ Chaotic AUR is already installed");
    }
}

fn ensure_yay_is_installed(root_shell: &mut RootShell) {
    if !pacman::is_installed("yay") {
        println!("✔️ Installing yay");
        pacman::install("yay", root_shell);
    } else {
        println!("✔️ yay is already installed");
    }
}

fn ensure_arch_based_distro() {
    if shell::execute_with_output("cat /etc/os-release | grep -i arch")
        .trim()
        .is_empty()
    {
        println!("❌ This app only works on arch based distros");
        std::process::exit(1);
    }

    // Print current os name
    println!(
        "✔️ Running on {}",
        shell::execute_with_output(
            "cat /etc/os-release | grep -i name | grep PRETTY_NAME | cut -d '=' -f 2"
        )
        .trim()
    );
}

fn ensure_non_root_privileges() {
    if shell::is_root() {
        println!("❌ Please run this app without root privileges");
        std::process::exit(1);
    }

    // Print current user
    println!(
        "✔️ Running as {}",
        shell::execute_with_output("whoami").trim()
    );
}

#[derive(Clone)]
pub struct FeatureGroup {
    pub name: String,
}

impl Feature for FeatureGroup {
    fn install(&self, _root_shell: &mut RootShell) -> bool {
        true
    }

    fn uninstall(&self, _root_shell: &mut RootShell) -> bool {
        true
    }

    fn is_installed(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_group_element(&self) -> bool {
        true
    }
}
