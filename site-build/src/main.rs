//! # Markdown to html
//!
//! This program converts a directory of markdown files to html files.
//! It also copies the directory structure of the source directory to the destination directory.
//! The destination directory is created if it does not exist.
//!

use std::env;
use std::fs;
use std::io;
mod manage_structure;
use manage_structure::*;
mod md_to_html;
use md_to_html::*;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let (src, out) = get_input()?;

    match copy_files(&src, &out) {
        Ok(_) => println!("Files and folders copied successfully!"),
        Err(e) => eprintln!("Error copying files and folders: {}", e),
    }

    let layout_path = src.to_str().unwrap().to_string() + "/layout/base.html";
    let html = match fs::read_to_string(layout_path) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error reading layout file: {}", e);
            return Err(e);
        }
    };

    match std::env::set_current_dir(&out) {
        Ok(_) => println!("Current directory changed successfully!"),
        Err(e) => eprintln!("Error changing current directory: {}", e),
    }

    dirs_walker(&PathBuf::from("./"))?
        .iter()
        .map(|path| files_walker(path))
        .filter(|files| files.is_ok())
        .map(|files| files.unwrap())
        .flatten()
        .filter(|path| path.extension().unwrap_or_default() == "md")
        .map(|path| path.strip_prefix(&out).unwrap().to_path_buf())
        .for_each(|path| match html_generator(&path, &html) {
            Ok(_) => println!("File {} converted successfully!", path.display()),
            Err(e) => eprintln!("Error converting file: {}", e),
        });

    Ok(())
}

fn get_input() -> Result<(PathBuf, PathBuf), io::Error> {
    let mut src = "../";
    let mut out = "../site";

    let args = env::args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-d" => out = args_iter.next().unwrap(),
            "-t" => src = args_iter.next().unwrap(),
            _ => {
                // help
                println!("Usage: {} [-d destination] [-t target]", args[0]);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
            }
        }
    }

    if !PathBuf::from(out).exists() {
        std::fs::create_dir_all(out)?;
    }

    Ok((
        PathBuf::from(src).canonicalize().unwrap(),
        PathBuf::from(out).canonicalize().unwrap(),
    ))
}
