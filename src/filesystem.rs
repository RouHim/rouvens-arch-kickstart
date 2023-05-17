use crate::shell::RootShell;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn replace_string_in_file(file_path: &PathBuf, search_string: &str, new_line: &str) -> bool {
    let content = fs::read_to_string(file_path).unwrap();

    let mut lines: Vec<String> = content.split('\n').map(|x| x.to_string()).collect();
    let index_to_manipulate = lines.binary_search(&search_string.to_string()).unwrap();
    lines[index_to_manipulate] = new_line.to_string();
    let new_content = lines.join("\n");

    fs::write(file_path, new_content).is_ok()
}

pub fn set_readonly(root_shell: &mut RootShell, file_path: &Path, readonly: bool) {
    if readonly {
        root_shell.execute(format!("chattr +i {}", file_path.to_str().unwrap()));
    } else {
        root_shell.execute(format!("chattr -i {}", file_path.to_str().unwrap()));
    }
}

pub fn write_string_to_file(file_path: &PathBuf, data: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    writeln!(file, "{}", data).expect("Failed to write to file");
}

pub fn download_file(url: &str, target_path: PathBuf) {
    let response = ureq::get(url).call().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut bytes).unwrap();
    fs::write(target_path, bytes).unwrap();
}
