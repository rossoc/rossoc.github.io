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

mod args;
mod compile;
mod error;
mod file_walker;
mod parser;
mod serve;
mod watcher;
use crate::parser::make_site;
use crate::serve::serve_directory;
use crate::watcher::exec_on_event;
use args::Args;
use chrono::Local;
use clap::Parser;
use file_walker::copy_dir_all;
use tokio::signal;
use tokio::task::spawn;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    copy_dir_all(&args.src, &args.out).unwrap();
    match make_site(&args.src, &args.out) {
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    let target = args.src.to_owned();
    let dest = args.out.to_owned();

    let compile_fn = move |_| -> () {
        copy_dir_all(&target, &dest).unwrap();
        match make_site(&target, &dest) {
            Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
            Err(e) => println!("error: {:?}", e),
        }
    };

    tokio::select! {
    _ = async {
        let mut res = spawn(async {});
        if args.watch {
            res = spawn(async move {
                match exec_on_event(&args.src, &compile_fn) {
                    Ok(()) => (),
                    Err(e) => eprintln!("{}", e),
                };
            });
        }
        if args.serve {
            let addr = ("127.0.0.1", args.port);
            let dest_cp = args.out.to_owned();
            serve_directory(addr, &dest_cp).await;
            println!(
                "serving {} on http://{}:{}",
                &args.out.display(),
                addr.0,
                addr.1
            );
        }
        let _ = res.await;
    } => {},
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
            std::process::exit(0)
        }
    };

    Ok(())
}
