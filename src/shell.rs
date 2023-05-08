use std::path::PathBuf;
use std::process::Command;

/// Execute a shell command as root using pkexec
pub fn execute_as_root(command: impl AsRef<str>) -> bool {
    println!("Executing as root: pkexec {}", command.as_ref());

    Command::new("sh")
        .arg("-c")
        .arg(format!("pkexec {}", command.as_ref()))
        .output()
        .expect("failed to execute process")
        .status
        .success()
}

/// Execute a shell command
pub fn execute(command: impl AsRef<str>) -> bool {
    println!("Executing: {}", command.as_ref());

    Command::new("sh")
        .arg("-c")
        .arg(command.as_ref())
        .output()
        .expect("failed to execute process")
        .status
        .success()
}

/// Detects if the current user is root.
pub fn is_root() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string().trim() == "root"
}

pub fn own_file_for_user(file: &str) -> bool {
    execute_as_root(format!("chown $SUDO_USER:$SUDO_USER {file}"))
}

pub fn execute_with_output(command: impl AsRef<str>) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.as_ref())
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn user_home_dir_path() -> PathBuf {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $HOME")
        .output()
        .expect("failed to execute process");

    PathBuf::from(String::from_utf8_lossy(&output.stdout).to_string().trim())
}
