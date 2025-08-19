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
mod serve;
mod watcher;
use crate::parser::make_site;
use crate::serve::serve_directory;
use crate::watcher::exec_on_event;
use chrono::Local;
use file_walker::copy_dir_all;
use std::env::args;
use std::path::PathBuf;
use tokio::signal;
use tokio::task::spawn;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (target, dest, watch, serve) = get_input();

    copy_dir_all(&target, &dest).unwrap();
    match make_site(&target, &dest) {
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    let target_cp = target.to_owned();
    let dest_cp = target.to_owned();

    let compile_fn = move |_| -> () {
        copy_dir_all(&target_cp, &dest_cp).unwrap();
        match make_site(&target_cp, &dest_cp) {
            Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
            Err(e) => println!("error: {:?}", e),
        }
    };

    let mut compile_handle = spawn(async move {});

    if watch {
        let _ = compile_handle.await;
        compile_handle = spawn(async move {
            match exec_on_event(&target, &compile_fn) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            };
        });
    }

    tokio::select! {
        _ = async {
            if serve {
                serve_directory(&dest, 8080).await?;
            }
            compile_handle.await?;
            Ok::<(), std::io::Error>(())
        } => {},
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
            std::process::exit(0)
        }
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
            "-s" => {
                src = args_iter
                    .next()
                    .expect("Error: -s flag requires a source directory path")
                    .to_string();
            }
            "-o" => {
                out = args_iter
                    .next()
                    .expect("Error: -o flag requires an output directory path")
                    .to_string();
            }
            "--watch" => {
                watch = true;
            }
            "--serve" => {
                serve = true;
            }
            _ => {
                eprintln!(
                    "Unknown argument: {}\n\n\
             Usage: {} [-s source] [-o output] [--watch] [--serve]\n\n\
             Options:\n\
             \t-s <path>     Set source directory (default: current directory)\n\
             \t-o <path>     Set output directory (default: ./build)\n\
             \t--watch       Watch for file changes and rebuild automatically\n\
             \t--serve       Serve output directory locally after building\n",
                    arg, args[0]
                );
                std::process::exit(1);
            }
        }
    }
    (src.into(), out.into(), watch, serve)
}
