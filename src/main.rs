mod bindkey;
mod chaoticaur;
mod pacman;
mod pacmanconfig;
mod pamac;
mod powerlevel10k;
mod shell;
mod zshautosuggestions;
mod zshcompletions;
mod zshrc;
mod zshsyntaxhighlighting;
mod gnomesystemmonitor;
mod gnomedashtopanel;
mod gnomeappindicator;
mod yay;
mod ui;
mod mouseaccleration;

#[cfg(test)]
mod chaoticaur_test;

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
