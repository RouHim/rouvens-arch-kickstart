mod bluetooth;
mod chaotic_aur;
mod docker;
mod emoji_support;
mod foot_terminal;
mod gnome_app_indicator;
mod gnome_dash_to_panel;
mod gnome_system_monitor;
mod mouse_acceleration;
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
}

fn main() {
    // Ensure that the user is running the script as root
    // And the home directory is not root
    // And the user is running an arch based distro
    ensure_root_privileges();
    ensure_home_directory_is_not_root();
    ensure_arch_based_distro();

    let features: Vec<Box<dyn Feature>> = vec![
        Box::new(zsh_completions::ZshCompletions {}),
        Box::new(zsh_syntax_highlighting::ZshSyntaxHighlighting {}),
        Box::new(zsh_autosuggestions::ZshAutoSuggestions {}),
        Box::new(zsh_powerlevel10k::ZshPowerLevel10k {}),
        Box::new(zsh_keybindings::ZshCommonKeyBindings {}),
        Box::new(gnome_system_monitor::GnomeShellSystemMonitor {}),
        Box::new(gnome_dash_to_panel::DashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(mouse_acceleration::DisableMouseAcceleration {}),
        Box::new(pamac::Pamac {}),
        Box::new(chaotic_aur::ChaoticAur {}),
        Box::new(bluetooth::Bluetooth {}),
        Box::new(docker::Docker {}),
        Box::new(terminator::Terminator {}),
        Box::new(foot_terminal::FootTerminal {}),
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
        shell::execute_with_output("cat /etc/os-release | grep -i name | grep PRETTY_NAME | cut -d '=' -f 2")
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
