use std::fs;
use std::path::{PathBuf};

pub fn replace_string_in_file(file_path: &PathBuf, search_string: &str, new_line: &str) -> bool {
    let content = fs::read_to_string(file_path).unwrap();

    let mut lines: Vec<String> = content.split('\n').map(|x| x.to_string()).collect();
    let mut index_to_manipulate = 0;
    for line in lines.clone() {
        if line.starts_with(search_string) {
            index_to_manipulate = lines.binary_search(&line).unwrap()
        }
    }

    lines.insert(index_to_manipulate, new_line.to_string());

    let new_content = lines.join("\n");

    fs::write(file_path, new_content).is_ok()
}
