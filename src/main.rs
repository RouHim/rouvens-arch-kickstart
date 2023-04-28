mod bindkey;
mod chaoticaur;
mod gnomeappindicator;
mod gnomedashtopanel;
mod gnomesystemmonitor;
mod mouseaccleration;
mod pacman;
mod pacmanconfig;
mod pamac;
mod powerlevel10k;
mod shell;
mod ui;
mod zshautosuggestions;
mod zshcompletions;
mod zshrc;
mod zshsyntaxhighlighting;

pub trait Feature {
    fn install(&self) -> bool;
    fn uninstall(&self) -> bool;
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
}

fn main() {
    let features: Vec<Box<dyn Feature>> = vec![
        Box::new(zshcompletions::ZshCompletions {}),
        Box::new(zshsyntaxhighlighting::ZshSyntaxHighlighting {}),
        Box::new(zshautosuggestions::ZshAutoSuggestions {}),
        Box::new(powerlevel10k::ZshPowerLevel10k {}),
        Box::new(bindkey::ZshBindKeys {}),
        Box::new(gnomesystemmonitor::GnomeShellSystemMonitor {}),
        Box::new(gnomedashtopanel::DashToPanel {}),
        Box::new(gnomeappindicator::GnomeShellExtensionAppIndicator {}),
        Box::new(mouseaccleration::DisableMouseAcceleration {}),
    ];

    ui::show(features).expect("Failed to run ui");
}
