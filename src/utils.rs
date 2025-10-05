use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn find_file_in_dir(file: &str, dir: &str) -> Option<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };

    for entry in entries.flatten() {
        let Ok(metadata) = entry.metadata() else {
            continue;
        };

        let is_executable = metadata.permissions().mode() & 0o111 != 0;
        let file_name_matches = entry.file_name() == file;

        if is_executable && file_name_matches {
            return Some(entry.path());
        }
    }
    None
}

pub fn find_executable(command: &str) -> Option<PathBuf> {
    let Ok(paths_var) = std::env::var("PATH") else {
        return None;
    };

    for dir in paths_var.split(':') {
        if let Some(path) = find_file_in_dir(command, dir) {
            return Some(path);
        }
    }
    None
}