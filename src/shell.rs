use std::path::PathBuf;
use std::process::Command;

/// Execute a shell command and return true if it was successful.
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

/// Execute a shell command and return the output if it was successful.
pub fn execute_with_output(command: impl AsRef<str>) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.as_ref())
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
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

/// Detects the current user's home directory based on the $SUDO_USER environment variable.
/// If $SUDO_USER is empty, then the current user is root and the home directory is /root.
/// Otherwise, the home directory is /home/$SUDO_USER.
pub fn sudo_user_home_dir() -> PathBuf {
    let output = &Command::new("sh")
        .arg("-c")
        .arg("echo $SUDO_USER")
        .output()
        .expect("failed to execute process")
        .stdout;
    let output_string = String::from_utf8_lossy(output).to_string();
    let sudo_user = output_string.trim();

    if sudo_user.is_empty() {
        PathBuf::from("/root")
    } else {
        PathBuf::from("/home").join(sudo_user)
    }
}

/// Detects the current user's name based on the $SUDO_USER environment variable.
/// If $SUDO_USER is empty, then the current user is root and the name is "root".
/// Otherwise, the name is $SUDO_USER.
pub fn sudo_user() -> String {
    let output = &Command::new("sh")
        .arg("-c")
        .arg("echo $SUDO_USER")
        .output()
        .expect("failed to execute process")
        .stdout;
    let output_string = String::from_utf8_lossy(output).to_string();
    let sudo_user = output_string.trim();

    if sudo_user.is_empty() {
        String::from("root")
    } else {
        String::from(sudo_user)
    }
}

/// Execute a shell command as the SUDE_USER.
pub fn execute_as_user(to_execute: &str) -> bool {
    execute(format!("su -c \"{to_execute}\" $SUDO_USER"))
}

pub fn execute_as_user_with_output(to_exec: &str) -> Option<String> {
    execute_with_output(format!("su -c \"{to_exec}\" $SUDO_USER"))
}