use std::fs;
use std::io;
use std::path::PathBuf;

const IGNORE_DIRS: [&str; 4] = [".git", ".github", "layout", "site-build"];

// Copy the files and folders from the source folder to the destination folder
pub fn copy_files(
    source_folder: &PathBuf,
    destination_folder: &PathBuf,
) -> io::Result<std::process::Output> {
    std::process::Command::new("cp")
        .arg("-r")
        .arg(source_folder)
        .arg(destination_folder.join(".."))
        .arg("&&")
        .arg("mv")
        .arg(destination_folder.join(".."))
        .arg(destination_folder)
        .output()
}

// return the list of directories contained in the source folder
// and recursively in its subfolders
pub fn dirs_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut walker = fs::read_dir(source_folder)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|entry| {
            entry.is_dir() && !IGNORE_DIRS.contains(&entry.file_name().unwrap().to_str().unwrap())
        })
        .map(|dir| dir.canonicalize().unwrap())
        .map(|dir| dir)
        .collect::<Vec<PathBuf>>();

    // recursively add subfolders
    walker.extend(
        walker
            .clone()
            .iter()
            .flat_map(|path| dirs_walker(&path).unwrap()),
    );
    Ok(walker)
}

// return the list of files contained in the source folder
// and recursively its subfolders
pub fn files_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let walker = fs::read_dir(source_folder)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.canonicalize().unwrap())
        .collect::<Vec<PathBuf>>();
    Ok(walker)
}
