use tempfile::tempdir;
use std::fs::File;
use std::io::{self, Write};
use std::Path::PathBuf;

use super::downloader::find_executables;

#[cfg(test)]
fn test_find_executable() {
    use std::path::PathBuf;

    let temporary_directory = tempdir()?;

    let temporary_executable = temporary_directory.path().join("temp.exe");
    let mut file = File::create(temporary_executable);

    let executables_found: Vec<PathBuf> = find_executables(temporary_directory);
    
    assert!(executables_found.len() == 1);

    for executable in executables_found {
        let mut file_pathbuf = PathBuf::from(temporary_directory.path().join("temp.exe"));
        assert!(executable == file_pathbuf);
    }

    drop(file);
    temporary_directory.close()?;
}