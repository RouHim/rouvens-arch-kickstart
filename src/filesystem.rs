
use std::fs;


use std::path::{PathBuf};

pub fn replace_string_in_file(file_path: &PathBuf, search_string: &str, new_line: &str) -> bool {
    let mut content = fs::read_to_string(file_path).unwrap();

    content = content.replace(search_string, new_line);

    fs::write(file_path, content).is_ok()
}

pub fn download_file(url: &str, target_path: PathBuf) {
    let response = ureq::get(url).call().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut bytes).unwrap();
    fs::write(target_path, bytes).unwrap();
}
