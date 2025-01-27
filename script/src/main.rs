//! # Markdown to html
//!
//! This program converts a directory of markdown files to html files.
//! The structure of the directory needs to be the following:
//! ```markdown
//! target
//! ├── folder1
//! │   ├── file1.md
//! │   ├── file2.md
//! │   └── ...
//! ├── layout
//! │   ├── some_layout.md
//! │   └── ...
//! ├── assets
//! │   ├── some_asset.md
//! │   └── ...
//! └── ...
//! ```
//!
//! Once the program is run, it will wait for an event to occur in the target
//! directory.
//!
//! ## Idee
//! - usare axum per creare un server che serve i file html (molto simile a actix-web)

mod md_to_html;
use chrono::Local;
use md_to_html::{md_to_html, IGNORE_FOLDERS};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let target = "../";
    let dest = "../_site/";

    copy_dir_all(&PathBuf::from(target), &PathBuf::from(dest))?;

    match md_to_html(&target, &dest) {
        // print DD/MM/YYYY-HH:MM:SS
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    match src.file_name() {
        Some(path) => {
            if IGNORE_FOLDERS.contains(&path.to_str().unwrap()) {
                return Ok(());
            }
        }
        None => {}
    }
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
