use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref ZSHRC_PATH: PathBuf = dirs::home_dir().unwrap().join(".zshrc");
}

pub fn add_line(line_to_append: &str) -> bool {
    if !line_exists(line_to_append) {
        append_to_file(line_to_append);
    }

    true
}

pub fn remove_line(line_to_remove: &str) -> bool {
    if let Ok(file) = OpenOptions::new().read(true).open(ZSHRC_PATH.as_path()) {
        let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(ZSHRC_PATH.as_path())
            .unwrap();

        for line in lines {
            if line.trim() != line_to_remove {
                writeln!(file, "{}", line).expect("Failed to write to file");
            }
        }

        true
    } else {
        false
    }
}

pub fn line_exists(line_to_find: &str) -> bool {
    if let Ok(file) = OpenOptions::new().read(true).open(ZSHRC_PATH.as_path()) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.trim() == line_to_find {
                return true;
            }
        }
    }

    false
}

fn append_to_file(line_to_append: &str) -> bool {
    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(ZSHRC_PATH.as_path())
    {
        writeln!(file, "{}", line_to_append).expect("Failed to write to file");
        true
    } else {
        false
    }
}
