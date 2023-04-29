mod zsh_keybindings;
mod bluetooth;
mod chaotic_aur;
mod gnome_app_indicator;
mod gnome_dash_to_panel;
mod gnome_system_monitor;
mod mouse_accleration;
mod pacman;
mod pacman_config;
mod pamac;
mod zsh_powerlevel10k;
mod shell;
mod terminator;
mod ui;
mod zsh_autosuggestions;
mod zsh_completions;
mod zshrc;
mod zsh_syntaxhighlighting;
mod docker;

pub trait Feature {
    fn install(&self) -> bool;
    fn uninstall(&self) -> bool;
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
}

fn main() {
    let features: Vec<Box<dyn Feature>> = vec![
        Box::new(zsh_completions::ZshCompletions {}),
        Box::new(zsh_syntaxhighlighting::ZshSyntaxHighlighting {}),
        Box::new(zsh_autosuggestions::ZshAutoSuggestions {}),
        Box::new(zsh_powerlevel10k::ZshPowerLevel10k {}),
        Box::new(zsh_keybindings::ZshCommonKeyBindings {}),
        Box::new(gnome_system_monitor::GnomeShellSystemMonitor {}),
        Box::new(gnome_dash_to_panel::DashToPanel {}),
        Box::new(gnome_app_indicator::GnomeShellExtensionAppIndicator {}),
        Box::new(mouse_accleration::DisableMouseAcceleration {}),
        Box::new(pamac::Pamac {}),
        Box::new(chaotic_aur::ChaoticAur {}),
        Box::new(bluetooth::Bluetooth {}),
        Box::new(terminator::Terminator {}),
        Box::new(docker::Docker {}),
    ];

    ui::show(features).expect("Failed to run ui");
}
