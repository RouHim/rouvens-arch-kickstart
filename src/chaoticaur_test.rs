use assertor::{assert_that, OptionAssertion};

use crate::chaoticaur;

#[test]
fn should_parse_correctly() {
    // GIVEN is nothing

    // WHEN parsing chaotic aur install commands
    let cmds: Option<Vec<String>> = chaoticaur::get_install_commands();

    // Then they should match
    assert_that!(cmds).is_some();
    assert_eq!(cmds.unwrap(), vec![
        "pacman-key --recv-key FBA220DFC880C036 --keyserver keyserver.ubuntu.com",
        "pacman-key --lsign-key FBA220DFC880C036",
        "pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'",
    ])
}
