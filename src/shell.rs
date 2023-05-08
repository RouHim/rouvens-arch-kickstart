use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct RootShell {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
}

impl RootShell {
    pub fn new() -> io::Result<Self> {
        let mut child = Command::new("pkexec")
            .arg("bash")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();

        Ok(Self { child, stdin })
    }

    pub fn execute(&mut self, command: impl AsRef<str>) {
        writeln!(self.stdin, "{}", command.as_ref()).unwrap();
        self.stdin.flush().unwrap();
    }
}

impl Drop for RootShell {
    fn drop(&mut self) {
        let _ = self.execute("exit");
        let _ = self.child.wait();
    }
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
