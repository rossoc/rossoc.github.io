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
mod watcher;
use crate::parser::make_site;
use crate::watcher::exec_on_event;
use chrono::Local;
use file_walker::copy_dir_all;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let target = PathBuf::from("./");
    let dest = PathBuf::from("./_site/");

    let function = |_| -> () {
        copy_dir_all(&target, &dest).unwrap();
        match make_site(&target, &dest) {
            Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
            Err(e) => println!("error: {:?}", e),
        }
    };

    match exec_on_event(&target, &function) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}
