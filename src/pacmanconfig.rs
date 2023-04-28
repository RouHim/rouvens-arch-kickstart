use std::io::BufRead;

const PACMAN_CONF: &str = "/etc/pacman.conf";
const CUSTOM_CONFIG: &str = "# CUSTOM CONFIG";

pub fn add_to_pacman_conf(config: &str) -> bool {
    let contents = std::fs::read_to_string(PACMAN_CONF).unwrap();
    if contents.contains(CUSTOM_CONFIG) {
        return true;
    }
    let contents = contents.trim_end();
    let new_contents = format!("{}\n\n{}\n{}", contents, CUSTOM_CONFIG, config);
    std::fs::write(PACMAN_CONF, new_contents).is_ok()
}

pub fn remove_from_pacman_conf() -> bool {
    let contents = std::fs::read_to_string(PACMAN_CONF).unwrap();
    let mut lines = contents.lines().collect::<Vec<_>>();
    let index = lines.iter().position(|&line| line == CUSTOM_CONFIG);
    if let Some(i) = index {
        lines.remove(i);
        std::fs::write(PACMAN_CONF, lines.join("\n")).is_ok()
    } else {
        true
    }
}

pub fn read_pacman_conf() -> Vec<String> {
    let contents = std::fs::read_to_string(PACMAN_CONF).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}
