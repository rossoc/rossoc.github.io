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
use std::env::args;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let (target, dest, watch, _) = get_input();

    copy_dir_all(&target, &dest).unwrap();
    match make_site(&target, &dest) {
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    let function = |_| -> () {
        copy_dir_all(&target, &dest).unwrap();
        match make_site(&target, &dest) {
            Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
            Err(e) => println!("error: {:?}", e),
        }
    };

    if watch {
        match exec_on_event(&target, &function) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        };
    }

    Ok(())
}

fn get_input() -> (PathBuf, PathBuf, bool, bool) {
    let mut src = "./".to_string();
    let mut out = "./_site".to_string();
    let mut watch = false;
    let mut serve = false;

    let args = args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-s" => src = args_iter.next().unwrap().to_string(),
            "-o" => out = args_iter.next().unwrap().to_string(),
            "--watch" => watch = true,
            "--serve" => serve = true,
            _ => {
                // help
                println!(
                    "Usage: {} [-s source] [-o output] [--watch] [--serve]\n entered {}, instead",
                    args[0], arg
                );
                std::process::exit(0);
            }
        }
    }
    (src.into(), out.into(), watch, serve)
}
