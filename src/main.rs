//! # Markdown to html
//!
//! This program converts a directory of Markdown files to HTML files.
//! The source directory is ignored, so you can have a `readme.md` and an
//! `index.html`. It recurses in every sub-directory.
//!
//! ```markdown
//! target
//! ├── folder1
//! │   ├── file1.md
//! │   ├── file2.md
//! │   └── folder2.md
//! │       ├── file3.md
//! │       ├── file4.md
//! │       └── ...
//! ├── layout
//! │   ├── some_layout.md
//! │   └── ...
//! ├── assets
//! │   ├── some_asset.jpg
//! │   └── ...
//! └── ...
//! ```
//!
//!
//!
//! Once the program is run, it will wait for an event to occur in the target
//! directory.
//!
//! ## Idee
//! - usare axum per creare un server che serve i file html (molto simile a actix-web)

mod compile;
mod error;
mod file_walker;
mod parser;
use crate::parser::make_site;
use chrono::Local;
use file_walker::copy_dir_all;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let target = "./";
    let dest = "./_site/";

    copy_dir_all(&PathBuf::from(target), &PathBuf::from(dest))?;

    match make_site(&target, &dest) {
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}
