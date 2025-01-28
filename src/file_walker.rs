use std::fs::{read_dir, ReadDir};
use std::io::Result;
use std::path::{Path, PathBuf};

fn ignore_files() -> Vec<String> {
    [
        "layout",
        "target",
        "src",
        ".git",
        ".github",
        ".obsidian",
        "script",
        "_site",
        ".gitignore",
    ]
    .iter()
    .map(|e| e.to_string())
    .collect::<_>()
}

/// Given a directory, returns the files in such directory and all
/// sub-directories recursively. Directories are considered files themselves.
/// Files returned by `ignore_files()` are ignored.
///
/// Input:
/// - source_dir: the source directory to start the recursion
pub fn files(source_dir: &Path) -> Result<Vec<PathBuf>> {
    let res = read_dir(source_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.path().canonicalize().ok())
        .filter(|e| match e.file_name() {
            Some(f) => match f.to_str() {
                Some(name) => !ignore_files().contains(&name.to_string()),
                None => false,
            },
            None => false,
        })
        .map(|e| match e.is_dir() {
            true => match files(&e) {
                Ok(mut res) => {
                    res.push(e);
                    res
                }
                Err(_) => vec![e],
            },
            false => vec![e],
        })
        .flatten()
        .collect::<_>();
    Ok(res)
}
