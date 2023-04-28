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