use std::boxed::Box;

use dyn_clone::DynClone;

use crate::chaotic_aur::ChaoticAur;
use crate::shell::RootShell;

mod chaotic_aur;
mod common_system_fixes;
mod docker;
mod filesystem;
mod fish_default_shell;
mod fs_trim;
mod gnome_app_indicator;
mod gnome_blur_my_shell;
mod gnome_dark_mode;
mod gnome_dash_to_panel;
mod gnome_date_menu_formatter;
mod gnome_dracula_gtk_theme;
mod gnome_just_perfection;
mod gnome_mouse_acceleration;
mod gnome_over_amplification;
mod gnome_shortcuts;
mod gnome_system_monitor;
mod gnome_tap_to_click;
mod gnome_tiling_assistant;
mod gnome_user_themes;
mod gnome_window_buttons;
mod hibernate_lid_closed;
mod kitty;
mod pacman;
mod pacman_config;
mod pacman_package;
mod pacman_package_service;
mod pacman_pamac;
mod remove_eos_welcome;
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
        Box::new(pacman_package_service::PacmanPackageService {
            package_name: "bluez bluez-utils",
            service_name: "bluetooth.service",
            description: "Install Bluetooth",
        }),
        Box::new(pacman_config::PacmanImprovements {}),
        Box::new(fs_trim::PeriodicTRIM {}),
        Box::new(docker::Docker {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "noto-fonts-emoji",
            description: "Install emoji support",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "ttf-fira-code",
            description: "Install fira code font",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "appimagelauncher",
            description: "Install AppImageLauncher",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "cpupower-gui",
            description: "Install cpupower-gui",
        }),
        Box::new(pacman_package_service::PacmanPackageService {
            package_name: "power-profiles-daemon",
            service_name: "power-profiles-daemon.service",
            description: "Install power-profiles-daemon",
        }),
        Box::new(common_system_fixes::CommonSystemFixes {}),
        Box::new(hibernate_lid_closed::HibernateWhenLidClosed {}),
        Box::new(remove_eos_welcome::RemoveEosWelcome {}),
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
        Box::new(gnome_user_themes::GnomeShellExtensionUserThemes {}),
        Box::new(gnome_date_menu_formatter::GnomeShellExtensionDateMenuFormatter {}),
        Box::new(gnome_system_monitor::GnomeShellExtensionSystemMonitor {}),
        Box::new(gnome_dash_to_panel::GnomeShellExtensionDashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(gnome_blur_my_shell::GnomeShellExtensionBlurMyShell {}),
        Box::new(gnome_just_perfection::GnomeShellExtensionJustPerfection {}),
        Box::new(gnome_tiling_assistant::GnomeShellExtensionTilingAssistant {}),
        Box::new(gnome_dracula_gtk_theme::GnomeDraculaGtkTheme {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "papirus-icon-theme",
            description: "Install Papirus Icons",
        }),
        Box::new(gnome_window_buttons::GnomeEnableWindowButtons {}),
        Box::new(gnome_over_amplification::GnomeOverAmplification {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gnome-tweaks",
            description: "Install gnome tweaks",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gnome-power-manager",
            description: "Install gnome power manager",
        }),
        // Apps
        Box::new(FeatureGroup {
            name: "Common Packages".to_string(),
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "firefox",
            description: "Install Firefox",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "vlc",
            description: "Install Vlc",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gnome-firmware",
            description: "Install gnome firmware updater",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "topgrade",
            description: "Install topgrade",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "menulibre",
            description: "Install menulibre",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "bottles",
            description: "Install bottles",
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
            description: "Install signal desktop",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "usbimager",
            description: "Install USB Imager",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "deja-dup",
            description: "Install Déjà Dup",
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
            package_name: "linux-zen linux-zen-headers",
            description: "Install Linux Zen Kernel",
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
            package_name: "vulkan-radeon vulkan-mesa-layers",
            description: "Install vulkan-radeon",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "dxvk-gplasync-bin",
            description: "Install Async DXVK",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "gamemode",
            description: "Install Feral GameMode",
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
        Box::new(pacman_package::PacmanPackage {
            package_name: "liquidctl",
            description: "Install Liquidctl",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "corectrl",
            description: "Install Corectrl",
        }),
        // Printing
        Box::new(FeatureGroup {
            name: "Printing".to_string(),
        }),
        Box::new(pacman_package_service::PacmanPackageService {
            package_name: "cups cups-pdf",
            service_name: "cups.service",
            description: "Install CUPS",
        }),
        Box::new(yay_package::YayPackage {
            package_name: "brother-mfc-j430w",
            description: "Install Brother MFC-J430W driver",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "brscan4",
            description: "Install brscan4",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "system-config-printer",
            description: "Install system-config-printer",
        }),
    ];

    ui::show(root_shell, features);
}

fn ensure_chaotic_aur_is_installed(root_shell: &mut RootShell) {
    let aur = ChaoticAur {};
    if !aur.is_installed() {
        println!("💫 Installing Chaotic AUR");
        aur.install(root_shell);
    } else {
        println!("✔️ Chaotic AUR is already installed");
    }
}

fn ensure_yay_is_installed(root_shell: &mut RootShell) {
    if !shell::execute("yay --version") {
        println!("💫 Installing yay");
        if !install_yay(root_shell) {
            println!("❌ Could not automatically install yay, please install it manually.");
            std::process::exit(1);
        }
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

/// Installs yay
fn install_yay(root_shell: &mut RootShell) -> bool {
    let required_packages = "git base-devel curl sed tar";

    if !pacman::is_installed(required_packages) {
        pacman::install(required_packages, root_shell);
    }

    let latest_tag = shell::execute_with_output(
        r#"curl -L -s -H 'Accept: application/json' https://github.com/Jguer/yay/releases/latest | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/ '"#,
    );
    let latest_version = latest_tag.as_str().trim().replace('v', "");
    shell::execute_with_output(format!("curl -L -o /tmp/yay.tar.gz https://github.com/Jguer/yay/releases/download/{latest_tag}/yay_{latest_version}_x86_64.tar.gz"));
    shell::execute("tar -xzf /tmp/yay.tar.gz -C /tmp/");
    shell::execute(format!(
        "/tmp/yay_{latest_version}_x86_64/yay -Sy --sudo pkexec --noconfirm yay-bin"
    ));
    shell::execute("rm -rf /tmp/yay*");
    shell::execute("yay --version")
}
