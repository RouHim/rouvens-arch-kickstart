mod bluetooth;
mod chaotic_aur;
mod docker;
mod emoji_support;
mod gnome_app_indicator;
mod gnome_dash_to_panel;
mod gnome_system_monitor;
mod gnome_mouse_acceleration;
mod pacman;
mod pacman_config;
mod pamac;
mod shell;
mod terminator;
mod ui;
mod zsh_autosuggestions;
mod zsh_completions;
mod zsh_keybindings;
mod zsh_powerlevel10k;
mod zsh_syntax_highlighting;
mod zshrc;

pub trait Feature {
    fn install(&self) -> bool;
    fn uninstall(&self) -> bool;
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
    fn is_group_element(&self) -> bool {
        false
    }
}

fn main() {
    // Ensure that the user is running the script as root
    // And the home directory is not root
    // And the user is running an arch based distro
    ensure_root_privileges();
    ensure_home_directory_is_not_root();
    ensure_arch_based_distro();

    let features: Vec<Box<dyn Feature>> = vec![
        // Shell
        Box::new(FeatureGroup {
            name: "Shell".to_string(),
        }),
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
        Box::new(gnome_system_monitor::GnomeShellSystemMonitor {}),
        Box::new(gnome_dash_to_panel::DashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(gnome_mouse_acceleration::GnomeDisableMouseAcceleration {}),
        // Pacman
        Box::new(FeatureGroup {
            name: "Pacman".to_string(),
        }),
        Box::new(pamac::Pamac {}),
        Box::new(chaotic_aur::ChaoticAur {}),
        // System
        Box::new(FeatureGroup {
            name: "System".to_string(),
        }),
        Box::new(bluetooth::Bluetooth {}),
        Box::new(docker::Docker {}),
        Box::new(emoji_support::EmojiSupport {}),
    ];

    ui::show(features).expect("Failed to run ui");
}

fn ensure_arch_based_distro() {
    if shell::execute_with_output("cat /etc/os-release | grep -i arch").is_none() {
        println!("❌ This app only works on arch based distros");
        std::process::exit(1);
    }

    // Print current os name
    println!(
        "✔️ Running on {}",
        shell::execute_with_output(
            "cat /etc/os-release | grep -i name | grep PRETTY_NAME | cut -d '=' -f 2"
        )
        .unwrap()
        .trim()
    );
}

fn ensure_home_directory_is_not_root() {
    if shell::sudo_user_home_dir()
        .to_str()
        .unwrap()
        .contains("root")
    {
        println!("❌ Please run this app with sudo: 'sudo rouvens-arch-kickstart'");
        std::process::exit(1);
    }

    // Print current user home directory
    println!(
        "✔️ User home dir: {}",
        shell::sudo_user_home_dir().to_str().unwrap()
    );
}

fn ensure_root_privileges() {
    if !shell::is_root() {
        println!("❌ Please run this app with sudo: 'sudo rouvens-arch-kickstart'");
        std::process::exit(1);
    }

    // Print current user
    println!("✔️ Running as root ({})", shell::sudo_user());
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
