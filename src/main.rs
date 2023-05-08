use crate::shell::RootShell;

mod bluetooth;
mod chaotic_aur;
mod docker;
mod gnome_app_indicator;
mod gnome_dark_mode;
mod gnome_dash_to_panel;
mod gnome_mouse_acceleration;
mod gnome_shortcuts;
mod gnome_system_monitor;
mod gnome_tap_to_click;
mod pacman;
mod pacman_package;
mod shell;
mod terminator;
mod ui;
mod zsh_autosuggestions;
mod zsh_completions;
mod zsh_default_shell;
mod zsh_keybindings;
mod zsh_powerlevel10k;
mod zsh_syntax_highlighting;
mod zshrc;

pub trait Feature {
    fn install(&self, root_shell: RootShell,) -> bool;
    fn uninstall(&self) -> bool;
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
    let mut root_shell = RootShell::new().unwrap();

    let features: Vec<Box<dyn Feature>> = vec![
        // Shell
        Box::new(FeatureGroup {
            name: "Shell".to_string(),
        }),
        Box::new(zsh_default_shell::ZshDefaultShell {}),
        Box::new(zsh_completions::ZshCompletions {}),
        Box::new(zsh_syntax_highlighting::ZshSyntaxHighlighting {}),
        Box::new(zsh_autosuggestions::ZshAutoSuggestions {}),
        Box::new(zsh_powerlevel10k::ZshPowerLevel10k {}),
        Box::new(zsh_keybindings::ZshCommonKeyBindings {}),
        Box::new(terminator::Terminator {}),

        // Gnome
        Box::new(FeatureGroup {
            name: "Gnome".to_string(),
        }),
        Box::new(gnome_dark_mode::GnomeDarkMode {}),
        Box::new(gnome_tap_to_click::GnomeTapToClick {}),
        Box::new(gnome_system_monitor::GnomeShellExtensionSystemMonitor {}),
        Box::new(gnome_dash_to_panel::GnomeShellExtensionDashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(gnome_mouse_acceleration::GnomeDisableMouseAcceleration {}),
        Box::new(gnome_shortcuts::GnomeKeyboardShortcuts {}),

        // Pacman
        Box::new(FeatureGroup {
            name: "Pacman".to_string(),
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "pamac",
            description: "Install Pamac",
        }),
        Box::new(chaotic_aur::ChaoticAur {}),

        // System
        Box::new(FeatureGroup {
            name: "System".to_string(),
        }),
        Box::new(bluetooth::Bluetooth { root_shell: &mut root_shell }),
        Box::new(docker::Docker {}),
        Box::new(pacman_package::PacmanPackage {
            package_name: "noto-fonts-emoji",
            description: "Install emoji support",
        }),

        // Apps
        Box::new(FeatureGroup {
            name: "System".to_string(),
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
            package_name: "gparted",
            description: "Install gparted",
        }),
        Box::new(pacman_package::PacmanPackage {
            package_name: "timeshift",
            description: "Install timeshift",
        }),
    ];

    ui::show(features).expect("Failed to run ui");
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

pub struct FeatureGroup {
    pub name: String,
}

impl Feature for FeatureGroup {
    fn install(&self) -> bool {
        true
    }

    fn uninstall(&self) -> bool {
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
